/**
 * @param {number} eventId
 * @param {object | URLSearchParams} payload
 * @param {Node} target
 * @param {AbortController | undefined} abortController
 * @param {WeakMap<HTMLElement, bool> | undefined} disabledBefore
 */
async function update(
  eventId,
  payload,
  target,
  abortController,
  disabledBefore,
) {
  if (this.abortController) {
    this.abortController.abort();
  }
  abortController = this.abortController =
    abortController ?? new AbortController();
  const signal = this.abortController.signal;
  if (signal.aborted) {
    return;
  }

  try {
    let state = undefined;
    if (
      target.firstChild &&
      target.firstChild instanceof HTMLScriptElement &&
      target.firstChild.getAttribute("type") === "application/json"
    ) {
      state = target.firstChild.innerText;
    }

    /** @type {RequestInit} */
    const req =
      payload instanceof URLSearchParams
        ? {
            signal,
            method: "PUT",
            headers: {
              "x-cabin": "boundary",
            },
            body: (() => {
              const formData = new FormData();
              formData.append(
                "payload",
                new Blob([payload.toString()], {
                  type: "application/x-www-form-urlencoded",
                }),
              );
              formData.append("event_id", eventId);
              if (state) {
                formData.append(
                  "state",
                  new Blob([state], { type: "application/json" }),
                );
              }
              return formData;
            })(),
          }
        : {
            signal,
            method: "PUT",
            headers: {
              "Content-Type": "application/json",
              "x-cabin": "boundary",
            },
            body: `{"eventId":${eventId},"payload":${JSON.stringify(payload)}${
              state ? `,"state":${state}` : ""
            }}`,
          };
    const endpoint =
      target instanceof CabinBoundary
        ? `/__boundary/${target.getAttribute("name")}`
        : location.href;
    const res = await fetch(endpoint, req);
    if (signal.aborted) {
      return;
    }

    const url = new URL(res.url);
    if (res.ok && res.redirected && url.pathname === "/client_redirect") {
      window.location = url.search.substring(1);
      return;
    }

    if (!target.parentNode) {
      return;
    }

    if (res.status !== 200) {
      throw new Error(`received unexpected status code: ${res.status}`);
    }

    const html = await res.text();

    console.time("patch");
    const template = document.createElement("template");
    template.innerHTML = html;
    patchChildren(target, template.content, {}, disabledBefore);
    console.timeEnd("patch");

    const rewriteUrl = res.headers.get("location");
    if (rewriteUrl && `${location.pathname}${location.search}` !== rewriteUrl) {
      history.pushState(null, undefined, rewriteUrl);
    }

    const newTitle = res.headers.get("x-cabin-title");
    if (newTitle) {
      document.title = newTitle;
    }

    // TODO: prevent endless event loops
    {
      const eventId = res.headers.get("x-cabin-event");
      const payload = res.headers.get("x-cabin-payload");
      if (eventId && payload) {
        target.dispatchEvent(
          new CustomEvent("cabinFire", {
            detail: { eventId, payload: JSON.parse(payload) },
            bubbles: true,
          }),
        );
      }
    }
  } catch (err) {
    if (err instanceof DOMException && err.name === "AbortError") {
      // ignore
    } else {
      throw err;
    }
  } finally {
    if (this.abortController === abortController) {
      this.abortController = undefined;
    }
  }
}

function setUpEventListener(el, eventName, opts) {
  const attrName = `cabin-${eventName}`;
  /** @type {WeakMap<Element, AbortController>} */
  const abortControllers = opts.abortControllers ?? new WeakMap();

  /**
   * @this {HTMLElement}
   * @param {Event} e
   */
  async function handleEvent(e) {
    /** @type {Element} */
    let node = e.target;

    do {
      const eventId = e.detail?.eventId ?? node.getAttribute(attrName);
      if (!eventId) {
        continue;
      }

      // The boundary only intercepts certain events
      if (opts.events && !opts.events.has(eventId)) {
        return;
      }

      // The internal state/view of the boundary is possibly going to change due to this event. To
      // force an update for the boundary if its parent view changes, remove all hash attributes
      // from ascendents up until the next cabin boundary.
      if (el !== document) {
        let el = this;
        do {
          el.removeAttribute("hash");
        } while ((el = el.parentElement) && !(el instanceof CabinBoundary));
      }

      if (opts.disable && node.disabled) {
        return;
      }

      // Force refresh everything from the target to its boundary to ensure that the server might
      // override e.g. input states.
      if (opts.dirty) {
        let el = node;
        do {
          el.removeAttribute("hash");
        } while ((el = el.parentElement) && !(el instanceof CabinBoundary));
      }

      {
        const abortController = abortControllers.get(node);
        if (abortController) {
          abortController.abort();
        }
      }

      const abortController = new AbortController();
      abortControllers.set(node, abortController);

      const isSubmitEvent = eventName === "submit";
      e.stopPropagation();
      if (
        (opts.preventDefault && !(node instanceof HTMLInputElement)) ||
        isSubmitEvent
      ) {
        e.preventDefault();
      }

      if (opts.debounce) {
        await new Promise((resolve) => setTimeout(resolve, opts.debounce));
        if (abortController.signal.aborted) {
          return;
        }
      }

      /** @type {WeakMap<HTMLElement, bool>} */
      const disabledBefore = new WeakMap();

      try {
        let payload;
        if (isSubmitEvent) {
          payload = new URLSearchParams(new FormData(node));
        } else if (opts?.eventPayload) {
          payload = JSON.parse(
            Object.entries(opts.eventPayload(e)).reduce(
              (result, [placeholder, value]) =>
                result.replace(placeholder, value),
              node.getAttribute(`${attrName}-payload`),
            ),
          );
        } else {
          payload =
            e.detail && typeof e.detail === "object" && "payload" in e.detail
              ? e.detail.payload
              : JSON.parse(node.getAttribute(`${attrName}-payload`));
        }

        if (isSubmitEvent) {
          // disable whole form
          for (const el of node.elements) {
            disabledBefore.set(el, el.disabled);
            el.disabled = true;
          }
        } else if (opts.disable) {
          disabledBefore.set(node, node.disabled);
          node.disabled = true;
        }

        // Check for, and if exists, apply pre-rendered instances of this boundary
        if (!isSubmitEvent && this instanceof CabinBoundary) {
          let templates = [];
          let template = this.lastElementChild;
          while (
            template &&
            template instanceof HTMLTemplateElement &&
            template.hasAttribute("event-id") &&
            template.hasAttribute("event-payload")
          ) {
            templates.push(template);
            template = template.previousElementSibling;
          }
          // TODO: might not always be the same, somehow use same string representation of the
          // payload as the server?
          const payloadStr = JSON.stringify(payload);
          for (const template of templates) {
            if (
              template.getAttribute("event-id") === eventId &&
              template.getAttribute("event-payload") === payloadStr
            ) {
              console.time("patch");
              patchChildren(el, template.content, {}, disabledBefore);
              // put back prerendered templates
              for (const template of templates) {
                this.appendChild(template);
              }
              console.timeEnd("patch");
              return;
            }
          }
        }

        await update(
          parseInt(eventId),
          payload,
          el == document ? document.body : el,
          abortController,
          disabledBefore,
        );
      } catch (err) {
        throw err;
      } finally {
        if (isSubmitEvent) {
          // restore disabled state
          for (const el of node.elements) {
            const before = disabledBefore.get(el);
            if (before !== undefined) {
              el.disabled = before;
            }
          }
        } else if (opts.disable) {
          node.disabled = disabledBefore.get(node) ?? false;
        }

        abortControllers.delete(node);
      }

      break;
    } while ((node = node.parentElement));
  }

  el.addEventListener(eventName, function (e) {
    handleEvent.call(this, e).catch((err) => {
      console.error(err);
    });
  });
}

/**
 * @param {Node} rootBefore
 * @param {Node} rootAfter
 * @param {Record<string, Node>} orphanKeyed
 * @param {WeakMap<HTMLElement, bool> | undefined} disabledBefore
 */
function patchChildren(rootBefore, rootAfter, orphanKeyed, disabledBefore) {
  // console.log("apply", rootBefore, rootAfter);

  let nodeBefore = rootBefore.firstChild;
  let nodeAfter = rootAfter.firstChild;

  // Neither node has children, done here
  if (!nodeBefore && !nodeAfter) {
    return;
  }

  /** @type {Node | null} */
  let nextBefore = null;
  /** @type {Node | null} */
  let nextAfter = null;

  do {
    nextBefore = null;
    nextAfter = null;
    // console.log(nodeBefore, "vs", nodeAfter);

    if (!nodeAfter) {
      // console.log("removed", nodeBefore);
      nextBefore = nodeBefore.nextSibling;
      rootBefore.removeChild(nodeBefore);
      continue;
    }

    // This node and all its next siblings are new nodes and can be directly added
    if (nodeBefore === null) {
      // console.log("append new", nodeAfter, "and siblings");
      const fragment = document.createDocumentFragment();
      let node = nodeAfter;
      while (node) {
        let next = node.nextSibling;
        if (isKeyedElement(node)) {
          // Only checking orphan nodes as there are no siblings remaining to check anyway
          const previous = orphanKeyed[node.id];
          if (previous) {
            // console.log(`found existing ${node.id} and moved it into place`);
            fragment.appendChild(previous);
            delete orphanKeyed[node.id];
            node = next;
            continue;
          }
        }
        fragment.appendChild(node);
        node = next;
      }

      rootBefore.appendChild(fragment);
      return;
    }

    // re-use if found somewhere else in the tree
    if (
      isKeyedElement(nodeAfter) &&
      (!isKeyedElement(nodeBefore) || nodeBefore.id !== nodeAfter.id)
    ) {
      const previous =
        document.getElementById(nodeAfter.id) ?? orphanKeyed[nodeAfter.id];
      nextBefore = nodeBefore;
      nextAfter = nodeAfter.nextSibling;
      if (
        previous &&
        (!previous.parentNode || previous.parentNode == nodeBefore.parentNode)
      ) {
        // console.log(`found existing ${nodeAfter.id} and moved it into place`);
        rootBefore.insertBefore(previous, nodeBefore);
        delete orphanKeyed[nodeAfter.id];
        nodeBefore = previous;
      } else {
        // console.log("new iter item, move new into place");
        rootBefore.insertBefore(nodeAfter, nodeBefore);
        continue;
      }
    }

    // type changed, replace completely
    else if (
      nodeBefore.nodeType !== nodeAfter.nodeType ||
      nodeBefore.nodeName !== nodeAfter.nodeName
    ) {
      // console.log("replace due to type change");
      nextBefore = nodeBefore.nextSibling;
      nextAfter = nodeAfter.nextSibling;
      rootBefore.replaceChild(nodeAfter, nodeBefore);

      // Keep it around in case it got moved
      if (isKeyedElement(nodeBefore)) {
        orphanKeyed[nodeBefore.id] = nodeBefore;
      }

      continue;
    }

    switch (nodeAfter.nodeType) {
      case Node.COMMENT_NODE:
        throw new Error("unexpected comment");

      case Node.ELEMENT_NODE:
        // skip sub-tree if hash is unchanged
        if (
          nodeBefore.hasAttribute("hash") &&
          nodeAfter.hasAttribute("hash") &&
          nodeBefore.getAttribute("hash") == nodeAfter.getAttribute("hash")
        ) {
          // console.log("skip due to unchanged hash");
          break;
        }

        // console.log("patch attributes");
        patchAttributes(nodeBefore, nodeAfter, disabledBefore);
        patchChildren(nodeBefore, nodeAfter, orphanKeyed, disabledBefore);
        break;

      case Node.TEXT_NODE:
        if (nodeAfter.textContent !== nodeBefore.textContent) {
          // console.log("update text");
          nodeBefore.textContent = nodeAfter.textContent;
        } else {
          // console.log("text is unchanged");
        }
        break;
    }
  } while (
    (nextAfter !== null || nextBefore !== null
      ? ((nodeAfter = nextAfter), (nodeBefore = nextBefore))
      : ((nodeAfter = nodeAfter?.nextSibling),
        (nodeBefore = nodeBefore?.nextSibling)),
    nodeAfter || nodeBefore)
  );
}

/**
 * @param {Element} childBefore
 * @param {Element} childAfter
 * @param {WeakMap<HTMLElement, bool> | undefined} disabledBefore
 */
function patchAttributes(childBefore, childAfter, disabledBefore) {
  // special handling for certain elements
  switch (childAfter.nodeName) {
    case "DIALOG":
      if (childAfter.hasAttribute("open")) {
        childBefore.show();
      } else {
        childBefore.close();
      }
      childAfter.removeAttribute("open");
  }

  const oldAttributeNames = new Set(childBefore.getAttributeNames());
  if (childBefore instanceof HTMLInputElement) {
    oldAttributeNames.add("checked");
    oldAttributeNames.add("selected");
    oldAttributeNames.add("value");
  }
  if (
    childBefore instanceof HTMLInputElement ||
    childBefore instanceof HTMLTextAreaElement
  ) {
    oldAttributeNames.add("disabled");
  }

  for (const name of childAfter.getAttributeNames()) {
    oldAttributeNames.delete(name);

    if (ignoreAttribute(childAfter, name)) {
      continue;
    }

    const newValue = childAfter.getAttribute(name);
    switch (name) {
      case "value":
        if (childBefore.value !== newValue) {
          // console.log("update attribute", name);
          childBefore.value = newValue;
        }
        break;
      case "checked":
        if (childBefore.checked !== childAfter.checked) {
          // console.log("update attribute", name);
          childBefore.checked = childAfter.checked;
        }
        break;
      case "selected":
        if (childBefore.selected !== childAfter.selected) {
          // console.log("update attribute", name);
          childBefore.selected = childAfter.selected;
        }
        break;
      case "disabled":
        disabledBefore?.set(childBefore, childAfter.disabled);
      // fallthrough
      default:
        if (childBefore.getAttribute(name) !== newValue) {
          // console.log("update attribute", name);
          childBefore.setAttribute(name, newValue);
        }
        break;
    }
  }

  // delete attributes that are not set anymore
  for (const name of oldAttributeNames) {
    if (ignoreAttribute(childBefore, name)) {
      continue;
    }
    // console.log("remove attribute", name);
    switch (name) {
      case "value":
        childBefore.value = "";
        break;
      case "checked":
        childBefore.checked = false;
        break;
      case "selected":
        childBefore.selected = false;
        break;
      case "disabled":
        childBefore.disabled = false;
        break;
      default:
        childBefore.removeAttribute(name);
        break;
    }
  }
}

/**
 * @param {Element} el
 * @param {string} attr
 */
function ignoreAttribute(el, attr) {
  switch (el.nodeName) {
    case "DIALOG":
      return attr === "open";
    default:
      return false;
  }
}

/**
 * @param {Node} node
 * @return {boolean}
 */
function isKeyedElement(node) {
  return node.nodeType === Node.ELEMENT_NODE && node.nodeName === "CABIN-KEYED";
}

function setupEventListeners(el) {
  let events =
    el instanceof CabinBoundary
      ? new Set(
          el
            .getAttribute("events")
            .split(",")
            .filter((s) => s.length > 0),
        )
      : null;
  if (events && events.size === 0) {
    events = undefined;
  }

  setUpEventListener(el, "click", {
    events,
    preventDefault: true,
    disable: true,
  });
  setUpEventListener(el, "transitionend", {
    events,
  });
  setUpEventListener(el, "animationend", {
    events,
  });
  setUpEventListener(el, "change", {
    events,
    eventPayload: (e) => ({
      "_##InputValue": e.target.value,
      '"_##InputChecked"': e.target.checked,
    }),
    dirty: true,
  });
  setUpEventListener(el, "input", {
    events,
    debounce: 500,
    eventPayload: (e) => ({
      "_##InputValue": e.target.value,
      '"_##InputChecked"': e.target.checked,
    }),
    dirty: true,
  });
  setUpEventListener(el, "submit", {
    events,
    preventDefault: true,
  });
  setUpEventListener(el, "cabinFire", {
    events,
  });
}

class CabinBoundary extends HTMLElement {
  constructor() {
    super();

    setupEventListeners(this);
  }
}

customElements.define("cabin-boundary", CabinBoundary);

setupEventListeners(document);

document.addEventListener("cabinRefresh", async function () {
  // Force update all boundary content
  for (let el of document.querySelectorAll("cabin-boundary")) {
    do {
      el.removeAttribute("hash");
    } while ((el = el.parentElement) && !(el instanceof CabinBoundary));
  }
  await update(0, {}, document.body);
});

document.addEventListener("cabinFire", async function (e) {
  await update(e.detail?.eventId, e.detail?.payload, document.body);
});

window.addEventListener("popstate", () => {
  document.dispatchEvent(new CustomEvent("cabinRefresh"));
});

window.addEventListener("pageshow", function (e) {
  // if loaded from cache, refresh page data
  if (e.persisted) {
    document.dispatchEvent(new CustomEvent("cabinRefresh"));
  }
});
