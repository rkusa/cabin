{
  const ENCODER = new TextEncoder();
  const DECODER = new TextDecoder();
  const WASM = { href: null, wasm: null };

  /**
   * @return {Promise<WebAssembly.WebAssemblyInstantiatedSource?}
   */
  async function loadWasm() {
    const path = document.querySelector("link[rel='cabin-components'][type='application/wasm']");
    if (path && path.href) {
      if (path.href === WASM.href) {
        return WASM.wasm;
      }

      try {
        const wasm = await WebAssembly.instantiateStreaming(fetch(path.href), {
          env: {
            error(msgPtr, msgLen) {
              loadWasm().then((wasm) => {
                const data = new Uint8Array(wasm.instance.exports.memory.buffer, msgPtr, msgLen);
                const msg = DECODER.decode(data);
                wasm.instance.exports.dealloc(msgPtr, msgLen);
                throw new Error(msg);
              });
            },
          },
        });
        wasm.instance.exports.init_panic_hook();

        WASM.href = path.href;
        WASM.wasm = wasm;

        return wasm;
      } catch (err) {
        console.error("failed to load components.wasm: ", err);
      }
    }

    return null;
  }

  /**
   * @param {string} eventId
   * @param {object | FormData} payload
   * @param {Node} target
   * @param {AbortController | undefined} abortController
   * @param {WeakMap<HTMLElement, bool> | undefined} disabledBefore
   */
  async function update(eventId, payload, target, abortController, disabledBefore) {
    if (typeof eventId !== "string") {
      throw new TypeError("event id must be a string");
    }

    const signal = abortController?.signal;
    if (signal?.aborted) {
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

      if (target instanceof CabinBoundary) {
        const name = target.getAttribute("name");
        const wasm = await loadWasm();
        if (wasm && wasm.instance.exports[name] && !(payload instanceof FormData)) {
          console.log("using wasm component");

          const event = `{"eventId":${JSON.stringify(eventId)},"payload":${JSON.stringify(payload)}${
            state ? `,"state":${state}` : ""
          }}`;
          const eventUtf8 = ENCODER.encode(event);
          const eventPtr = wasm.instance.exports.alloc(eventUtf8.length);
          new Uint8Array(wasm.instance.exports.memory.buffer, eventPtr, eventUtf8.length).set(
            eventUtf8,
          );
          const outPtr = wasm.instance.exports.alloc(4);
          const len = wasm.instance.exports[name](eventPtr, eventUtf8.length, outPtr);
          wasm.instance.exports.dealloc(eventPtr, eventUtf8.length);
          if (len > 0) {
            const view = new DataView(wasm.instance.exports.memory.buffer);
            const ptr = view.getInt32(outPtr, true);
            wasm.instance.exports.dealloc(outPtr, 4);

            const html = DECODER.decode(wasm.instance.exports.memory.buffer.slice(ptr, ptr + len));
            wasm.instance.exports.dealloc(ptr, len);

            console.time("patch");
            const template = document.createElement("template");
            template.innerHTML = html;
            patchChildren(target, template.content, {}, disabledBefore);
            console.timeEnd("patch");

            return;
          } else {
            console.warn("using wasm component failed, falling back to using serveer component");
          }
        } else if (wasm && wasm.instance.exports[name] && payload instanceof FormData) {
          console.warn("cannot use wasm components for form submissions (yet)");
        }
      }

      /** @type {RequestInit} */
      const req = (() => {
        if (payload instanceof FormData) {
          const formData = new FormData();
          formData.append("event_id", eventId);
          formData.append("state", new Blob([state ?? ""], { type: "application/json" }));
          formData.append(
            "payload",
            new Blob(
              [
                new URLSearchParams(
                  Array.from(payload.entries()).filter(([, v]) => !(v instanceof File)),
                ).toString(),
              ],
              {
                type: "application/x-www-form-urlencoded",
              },
            ),
          );
          for (const [k, v] of payload) {
            if (v instanceof File) {
              formData.append(k, v);
            }
          }
          return {
            signal,
            method: "PUT",
            headers: {
              "x-cabin": "boundary",
            },
            body: formData,
          };
        } else {
          return {
            signal,
            method: "PUT",
            headers: {
              "Content-Type": "application/json",
              "x-cabin": "boundary",
            },
            body: `{"eventId":${JSON.stringify(eventId)},"payload":${JSON.stringify(payload)}${
              state ? `,"state":${state}` : ""
            }}`,
          };
        }
      })();
      const endpoint =
        target instanceof CabinBoundary
          ? `/__boundary/${target.getAttribute("name")}`
          : location.href;
      const res = await fetch(endpoint, req);
      if (signal?.aborted) {
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

      if (res.status === 204) {
        const eventId = res.headers.get("cabin-event");
        const payload = res.headers.get("cabin-event-payload");
        if (eventId && payload) {
          target.dispatchEvent(
            new CustomEvent("cabinFire", {
              detail: { eventId, payload: JSON.parse(payload) },
              bubbles: true,
            }),
          );
        }
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
    } catch (err) {
      if (err instanceof DOMException && err.name === "AbortError") {
        // ignore
      } else {
        throw err;
      }
    }
  }

  /**
   * @param {HTMLElement} el
   * @param {string} eventName
   * @param {Object} opts
   * @param {Set<string>?} opts.events - list of events the boundary should handle
   * @param {bool?} opts.preventDefault - whether the default of the event should be prevented
   * @param {bool?} opts.disable - whether the element should be disabled while the event is handled
   * @param {bool?} opts.disableForm - whether to disable the target's form (but only up to the next
   * boundary)
   * @param {(e: Event) => Record<string, any>} opts.eventPayload - custom event payload
   * @param {number} opts.debounce - if set, the event execution is debounced by the given
   * milliseconds
   */
  function setUpEventListener(el, eventName, opts) {
    const attrName = `cabin-${eventName}`;

    /**
     * @this {HTMLElement}
     * @param {Event} e
     */
    async function handleEvent(e) {
      /** @type {Element} */
      let node = e.target;
      /** @type {string | undefined} */
      let eventId = undefined;

      // Find first ascendant that defines the event
      do {
        eventId =
          e.detail?.eventId ?? e.submitter?.getAttribute(attrName) ?? node.getAttribute(attrName);
        if (!eventId) {
          continue;
        } else {
          break;
        }
      } while ((node = node.parentElement));
      if (!node) {
        return;
      }

      // The boundary only intercepts certain events
      if (opts.events && !opts.events.has(eventId)) {
        return;
      }

      if (opts.disable && node.disabled) {
        return;
      }

      // The internal state/view of the boundary is possibly going to change due to this event. To
      // force an update for the boundary if any ascendent changes, remove their hash attributes.
      if (el !== document) {
        let el = this;
        do {
          el.removeAttribute("hash");
        } while ((el = el.parentElement));
      }

      // Note: `e.preventDefault()` doesn't seem to take full effect when called after
      // `abortController.abort()`, so ensure it stays before that
      e.stopPropagation();
      const isSubmitEvent = eventName === "submit";
      if ((opts.preventDefault && !(node instanceof HTMLInputElement)) || isSubmitEvent) {
        e.preventDefault();
      }

      // Only one concurrent event execution per boundary
      if (this.abortController) {
        this.abortController.abort();
      }

      /** @type {Map<HTMLElement, bool>} */
      const disabledBefore = new Map();
      /** @type {Map<HTMLElement, bool>} */
      const readOnlyBefore = new Map();

      const abortController = (this.abortController = new AbortController());
      abortController.abort = function () {
        if (abortController.signal.aborted) {
          return;
        }

        AbortController.prototype.abort.call(this);

        // restore disabled states
        for (const [el, before] of disabledBefore) {
          if (el.parentNode && before !== undefined) {
            el.disabled = before;
          }
        }

        // restore readOnly states
        for (const [el, before] of readOnlyBefore) {
          if (el.parentNode && before !== undefined) {
            el.readOnly = before;
          }
        }
      };

      // Do not make the parent function async to ensure e.preventDefault() and e.stopPropagation()
      // are not postponed by the promise.
      if (opts.debounce) {
        await new Promise((resolve) => setTimeout(resolve, opts.debounce));
        if (abortController.signal.aborted) {
          return;
        }
      }

      try {
        let payload;
        if (isSubmitEvent && !node.hasAttribute(`${attrName}-payload`)) {
          payload = new FormData(node);
        } else if (opts?.eventPayload) {
          payload = JSON.parse(
            Object.entries(opts.eventPayload(e)).reduce(
              (result, [placeholder, value]) => result.replace(placeholder, value),
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
        } else if (opts.disableForm) {
          /** @type {HTMLFormElement?} */
          let form = isSubmitEvent ? node : node.form;
          if (form?.elements) {
            for (const el of form.elements) {
              if (this.contains(el)) {
                if ("readOnly" in el.constructor.prototype) {
                  readOnlyBefore.set(el, el.readOnly);
                  el.readOnly = true;
                } else {
                  disabledBefore.set(el, el.disabled);
                  el.disabled = true;
                }
              }
            }
          }
        } else if (opts.disable) {
          disabledBefore.set(node, node.disabled);
          node.disabled = true;
        }

        // Don't change checkbox/radio inputs right away, as the server is going to decide the
        // outcome
        if (
          node instanceof HTMLInputElement &&
          (node.type === "checkbox" || node.type === "radio")
        ) {
          node.checked = !node.checked;
        }

        await update(
          eventId,
          payload,
          el == document ? document.body : el,
          abortController,
          disabledBefore,
          readOnlyBefore,
        );

        if (isSubmitEvent && node.parentNode) {
          node.reportValidity();
        }
      } catch (err) {
        throw err;
      } finally {
        abortController.abort(); // restore disabled states
      }
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
   * @param {WeakMap<HTMLElement, bool> | undefined} readOnlyBefore
   */
  function patchChildren(rootBefore, rootAfter, orphanKeyed, disabledBefore, readOnlyBefore) {
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
        const previous = document.getElementById(nodeAfter.id) ?? orphanKeyed[nodeAfter.id];
        nextBefore = nodeBefore;
        nextAfter = nodeAfter.nextSibling;
        if (previous && (!previous.parentNode || previous.parentNode == nodeBefore.parentNode)) {
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
          patchAttributes(nodeBefore, nodeAfter, disabledBefore, readOnlyBefore);
          patchChildren(nodeBefore, nodeAfter, orphanKeyed, disabledBefore, readOnlyBefore);
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
        : ((nodeAfter = nodeAfter?.nextSibling), (nodeBefore = nodeBefore?.nextSibling)),
      nodeAfter || nodeBefore)
    );
  }

  /**
   * @param {Element} childBefore
   * @param {Element} childAfter
   * @param {WeakMap<HTMLElement, bool> | undefined} disabledBefore
   * @param {WeakMap<HTMLElement, bool> | undefined} readOnlyBefore
   */
  function patchAttributes(childBefore, childAfter, disabledBefore, readOnlyBefore) {
    const oldAttributeNames = new Set(childBefore.getAttributeNames());
    if (childBefore instanceof HTMLInputElement) {
      if (childBefore.type === "checkbox" || childBefore.type === "radio") {
        oldAttributeNames.add("checked");
      } else {
        oldAttributeNames.add("value");
      }
    } else if (childBefore instanceof HTMLOptionElement) {
      oldAttributeNames.add("selected");
    }
    if (childBefore instanceof HTMLInputElement || childBefore instanceof HTMLTextAreaElement) {
      oldAttributeNames.add("disabled");
      oldAttributeNames.add("readonly");
    }

    for (const name of childAfter.getAttributeNames()) {
      oldAttributeNames.delete(name);

      if (ignoreAttribute(childAfter, name)) {
        continue;
      }

      const newValue = childAfter.getAttribute(name);
      switch (name) {
        case "value":
          // Using `.getAttribute("value")` instead of `.value` to keep local state unless it is an
          // intentional change from the server.
          if (childBefore.getAttribute("value") !== newValue) {
            // console.log("update attribute", name);
            childBefore.setAttribute("value", newValue);
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
          break;
        case "readonly":
          readOnlyBefore?.set(childBefore, childAfter.readOnly);
          break;
        default:
          if (childBefore.getAttribute(name) !== newValue) {
            // console.log("update attribute", name);
            childBefore.setAttribute(name, newValue);
          }
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
          disabledBefore?.set(childBefore, false);
          break;
        case "readonly":
          childBefore.readOnly = false;
          readOnlyBefore?.set(childBefore, false);
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

  /**
   * @param {HTMLElement} el
   */
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

    setUpEventListener(el, "mouseup", {
      events,
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
      disableForm: true,
    });
    setUpEventListener(el, "input", {
      events,
      debounce: 500,
      eventPayload: (e) => ({
        "_##InputValue": e.target.value,
        '"_##InputChecked"': e.target.checked,
      }),
    });
    setUpEventListener(el, "submit", {
      events,
      preventDefault: true,
      disableForm: true,
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
    await update("", {}, document.body);
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
}
