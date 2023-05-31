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
        // console.log("found", node);
        e.stopPropagation();

        if (opts.preventDefault) {
          e.preventDefault();
        }

        // TODO: abort on unmount
        if (this.abortController) {
          console.log("abort");
          this.abortController.abort();
        }
        const abortController = (this.abortController = new AbortController());
        const signal = this.abortController.signal;

        if (opts.disable) {
          node.disabled = true;
        }
        try {
          // const component = this.dataset.id;
          const payload = JSON.parse(
            opts?.eventPayload
              ? Object.entries(opts.eventPayload(e)).reduce(
                  (result, [placeholder, value]) =>
                    result.replace(placeholder, value),
                  node.getAttribute(`${attrName}-payload`)
                )
              : node.getAttribute(`${attrName}-payload`)
          );

          const state = document.getElementById("state").innerText;

          const res = await fetch(location.pathname, {
            signal,
            method: "PUT",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              eventId: parseInt(eventId),
              payload,
              // TODO: avoid JSON.parse to stringify it again right away
              state: JSON.parse(state),
            }),
          });
          if (signal.aborted) {
            console.log("already aborted, ignoring");
            return;
          }

          // TODO: handle status code
          const html = await res.text();
          // TODO: check if still mounted
          const template = document.createElement("template");
          template.innerHTML = html;
          patchChildren(document.body, template.content, {});
        } catch (err) {
          if (err instanceof DOMException && err.name === "AbortError") {
            // ignore
          } else {
            throw err;
          }
        } finally {
          if (opts.disable) {
            node.disabled = false;
          }
          if (this.abortController === abortController) {
            this.abortController = undefined;
          }
        }

        break;
      }
    } while ((node = node.parentElement) !== this);
  }

  document.addEventListener(eventName, handleEvent);
}

setUpEventListener("click", { preventDefault: true, disable: true });
setUpEventListener("input", {
  eventPayload: (e) => ({ "_##InputValue": e.target.value }),
});

/**
 * @param {Node} rootBefore
 * @param {Node} rootAfter
 * @param {Record<string, Node>} orphanComponents
 */
function patchChildren(rootBefore, rootAfter, orphanComponents) {
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

      // if (nodeBefore.nodeName === "SERVER-COMPONENT") {
      //   orphanComponents[nodeBefore.id] = nodeBefore;
      // }

      continue;
    }

    // This node and all its next siblings are new nodes and can be directly added
    if (nodeBefore === null) {
      console.log("append new", nodeAfter, "and siblings");
      const fragment = document.createDocumentFragment();
      let node = nodeAfter;
      while (node) {
        let next = node.nextSibling;
        fragment.appendChild(node);
        node = next;
      }

      rootBefore.appendChild(fragment);
      return;
    }

    // // component id changed, try to find existing one in DOM or replace completely
    // if (
    //   nodeAfter.nodeName === "SERVER-COMPONENT" &&
    //   (nodeBefore.nodeName !== "SERVER-COMPONENT" ||
    //     (nodeBefore.dataset.id === nodeAfter.dataset.id &&
    //       nodeBefore.id !== nodeAfter.id))
    // ) {
    //   const existing =
    //     document.getElementById(nodeAfter.id) ?? orphanComponents[nodeAfter.id];
    //   if (existing) {
    //     console.log("swap in existing component");
    //     if (nodeBefore.nodeName === "SERVER-COMPONENT") {
    //       existing.parentNode?.insertBefore(
    //         document.createComment("placeholder"),
    //         existing
    //       );
    //       rootBefore.replaceChild(existing, nodeBefore);
    //       orphanComponents[nodeBefore.id] = nodeBefore;
    //     } else if (
    //       nodeBefore.nodeType === Node.COMMENT_NODE &&
    //       nodeBefore.nodeValue === "placeholder"
    //     ) {
    //       rootBefore.replaceChild(existing, nodeBefore);
    //     } else {
    //       rootBefore.insertBefore(existing, nodeBefore);
    //     }
    //     nodeBefore = existing;
    //   }
    // }

    // type changed, replace completely
    if (
      nodeBefore.nodeType !== nodeAfter.nodeType ||
      nodeBefore.nodeName !== nodeAfter.nodeName
    ) {
      console.log("replace due to type change");
      nextBefore = nodeBefore.nextSibling;
      nextAfter = nodeAfter.nextSibling;
      rootBefore.replaceChild(nodeAfter, nodeBefore);

      // if (nodeBefore.nodeName === "SERVER-COMPONENT") {
      //   orphanComponents[nodeBefore.id] = nodeBefore;
      // }

      continue;
    }

    switch (nodeAfter.nodeType) {
      case Node.COMMENT_NODE:
        throw new Error("unexpected comment");

      case Node.ELEMENT_NODE:
        // TODO: tag changed
        console.log("patch attributes");
        patchAttributes(nodeBefore, nodeAfter);
        patchChildren(nodeBefore, nodeAfter, orphanComponents);
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
  const oldAttributeNames = new Set(childBefore.getAttributeNames());
  for (const name of childAfter.getAttributeNames()) {
    oldAttributeNames.delete(name);

    // TODO: handle new attributes

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
    console.log("remove attribute", name);
    childBefore.removeAttribute(name);
  }
}
