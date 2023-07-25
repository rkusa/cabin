/**
 * @param {number} eventId
 * @param {object | URLSearchParams} payload
 * @param {Node} target
 */
// TODO: remove `console.log`s or add option to disable them
async function update(eventId, payload, target) {
  if (this.abortController) {
    console.log("abort");
    this.abortController.abort();
  }
  const abortController = (this.abortController = new AbortController());
  const signal = this.abortController.signal;

  try {
    const state = document.getElementById("state").innerText;

    /** @type {RequestInit} */
    const req =
      payload instanceof URLSearchParams
        ? {
            signal,
            method: "PUT",
            body: (() => {
              const formData = new FormData();
              formData.append(
                "payload",
                new Blob([payload.toString()], {
                  type: "application/x-www-form-urlencoded",
                }),
              );
              formData.append("event_id", eventId);
              formData.append(
                "state",
                new Blob([state], { type: "application/json" }),
              );
              return formData;
            })(),
          }
        : {
            signal,
            method: "PUT",
            headers: {
              "Content-Type": "application/json",
            },
            body: `{"eventId":${eventId},"payload":${JSON.stringify(
              payload,
            )},"state":${state}}`,
          };
    const res = await fetch(location.href, req);
    if (signal.aborted) {
      console.log("already aborted, ignoring");
      return;
    }

    if (res.redirected && res.ok) {
      window.location = res.url;
      return;
    }

    if (!target.parentNode) {
      console.log("update target not mounted anymore, ignoring");
      return;
    }

    if (res.status !== 200) {
      throw new Error(`received unexpected status code: ${res.status}`);
    }

    const html = await res.text();

    console.time("patch");
    const template = document.createElement("template");
    template.innerHTML = html;
    patchChildren(target, template.content, {});
    console.timeEnd("patch");
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

document.addEventListener("cabinRefresh", async function () {
  await update(0, {}, document.body);
});

function setUpEventListener(eventName, opts) {
  const attrName = `cabin-${eventName}`;

  /**
   * @param {Event} e
   */
  async function handleEvent(e) {
    /** @type {Element} */
    let node = e.target;
    do {
      const eventId = node.getAttribute(attrName);
      if (eventId && (!opts.disable || !node.disabled)) {
        e.stopPropagation();

        if (opts.preventDefault) {
          e.preventDefault();
        }

        if (opts.disable) {
          node.disabled = true;
        }

        try {
          const payload = JSON.parse(
            opts?.eventPayload
              ? Object.entries(opts.eventPayload(e)).reduce(
                  (result, [placeholder, value]) =>
                    result.replace(placeholder, value),
                  node.getAttribute(`${attrName}-payload`),
                )
              : node.getAttribute(`${attrName}-payload`),
          );
          await update(parseInt(eventId), payload, document.body);
        } finally {
          if (opts.disable) {
            node.disabled = false;
          }
        }

        break;
      }
    } while ((node = node.parentElement));
  }

  document.addEventListener(eventName, handleEvent);
}

setUpEventListener("click", { preventDefault: true, disable: true });
setUpEventListener("input", {
  eventPayload: (e) => ({ "_##InputValue": e.target.value }),
});
setUpEventListener("change", {
  eventPayload: (e) => ({ "_##InputValue": e.target.value }),
});
document.addEventListener("submit", async function handleEvent(e) {
  /** @type {HTMLFormElement} */
  let form = e.target;
  do {
    const eventId = form.getAttribute("cabin-submit");
    if (!eventId) {
      return;
    }

    e.stopPropagation();
    e.preventDefault();

    const payload = new URLSearchParams(new FormData(form));

    // disable whole form
    /** @type {WeakMap<HTMLElement, bool>} */
    const disabledBefore = new WeakMap();
    for (const el of form.elements) {
      disabledBefore.set(el, el.disabled);
      el.disabled = true;
    }

    try {
      await update(parseInt(eventId), payload, document.body);
    } finally {
      // restore disabled state
      for (const el of form.elements) {
        const before = disabledBefore.get(el);
        if (before !== undefined) {
          el.disabled = before;
        }
      }
    }

    break;
  } while ((form = form.parentElement));
});

/**
 * @param {Node} rootBefore
 * @param {Node} rootAfter
 * @param {Record<string, Node>} orphanKeyed
 */
function patchChildren(rootBefore, rootAfter, orphanKeyed) {
  console.log("apply", rootBefore, rootAfter);

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
    console.log(nodeBefore, "vs", nodeAfter);

    if (!nodeAfter) {
      console.log("removed", nodeBefore);
      nextBefore = nodeBefore.nextSibling;
      rootBefore.removeChild(nodeBefore);
      continue;
    }

    // This node and all its next siblings are new nodes and can be directly added
    if (nodeBefore === null) {
      console.log("append new", nodeAfter, "and siblings");
      const fragment = document.createDocumentFragment();
      let node = nodeAfter;
      while (node) {
        let next = node.nextSibling;
        if (isKeyedElement(node)) {
          const previous =
            document.getElementById(node.id) ?? orphanKeyed[node.id];
          if (previous) {
            console.log(`found existing ${node.id} and moved it into place`);
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

    // re-use if found somewhere else in the three
    if (
      isKeyedElement(nodeAfter) &&
      (!isKeyedElement(nodeBefore) || nodeBefore.id !== nodeAfter.id)
    ) {
      const previous =
        document.getElementById(nodeAfter.id) ?? orphanKeyed[nodeAfter.id];
      nextBefore = nodeBefore;
      nextAfter = nodeAfter.nextSibling;
      if (previous) {
        console.log(`found existing ${nodeAfter.id} and moved it into place`);
        rootBefore.insertBefore(previous, nodeBefore);
        delete orphanKeyed[nodeAfter.id];
      } else {
        console.log("new iter item, move new into place");
        rootBefore.insertBefore(nodeAfter, nodeBefore);
        continue;
      }
    }

    // type changed, replace completely
    else if (
      nodeBefore.nodeType !== nodeAfter.nodeType ||
      nodeBefore.nodeName !== nodeAfter.nodeName
    ) {
      console.log("replace due to type change");
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
          console.log("skip due to unchanged hash");
          break;
        }

        console.log("patch attributes");
        patchAttributes(nodeBefore, nodeAfter);
        patchChildren(nodeBefore, nodeAfter, orphanKeyed);
        break;

      case Node.TEXT_NODE:
        if (nodeAfter.textContent !== nodeBefore.textContent) {
          console.log("update text");
          nodeBefore.textContent = nodeAfter.textContent;
        } else {
          console.log("text is unchanged");
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
 */
function patchAttributes(childBefore, childAfter) {
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
  for (const name of childAfter.getAttributeNames()) {
    oldAttributeNames.delete(name);

    if (ignoreAttribute(childAfter, name)) {
      continue;
    }

    const newValue = childAfter.getAttribute(name);
    if (childBefore.getAttribute(name) !== newValue) {
      console.log("update attribute", name);
      switch (name) {
        case "value":
          childBefore.value = newValue;
          break;
        default:
          childBefore.setAttribute(name, newValue);
          break;
      }
    }
  }

  // delete attributes that are not set anymore
  for (const name in oldAttributeNames) {
    if (ignoreAttribute(childBefore, name)) {
      continue;
    }
    console.log("remove attribute", name);
    childBefore.removeAttribute(name);
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
