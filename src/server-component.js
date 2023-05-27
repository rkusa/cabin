class ServerComponent extends HTMLElement {
  constructor() {
    super();

    // TODO: handle missing
    const initial = JSON.parse(this.lastElementChild.textContent);
    this.removeChild(this.lastElementChild);

    this.state = initial.state;
    this.hashTree = initial.hashTree;
    console.log(this.hashTree);

    this.setUpEventListener("click", { preventDefault: true, disable: true });
    this.setUpEventListener("input", {
      eventPayload: (e) => ({ "_##InputValue": e.target.value }),
    });

    // If the browser restored previous form values, detect them and trigger an input event
    const inputs = this.querySelectorAll("input");
    inputs.forEach((input) => {
      if (input.value !== input.getAttribute("value")) {
        console.log("input", input, "changed, manually trigger input event");
        input.dispatchEvent(
          new Event("input", { bubbles: true, cancelable: true })
        );
      }
    });
  }

  setUpEventListener(eventName, opts) {
    this.addEventListener(eventName, async (e) => {
      let node = e.target;
      do {
        if (node.dataset[eventName] && (!opts.disable || !node.disabled)) {
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
          const abortController = (this.abortController =
            new AbortController());
          const signal = this.abortController.signal;

          if (opts.disable) {
            node.disabled = true;
          }
          try {
            const component = this.dataset.id;
            const event = JSON.parse(
              opts?.eventPayload
                ? Object.entries(opts.eventPayload(e)).reduce(
                    (result, [placeholder, value]) =>
                      result.replace(placeholder, value),
                    node.dataset[eventName]
                  )
                : node.dataset[eventName]
            );

            // Collect descendant components and update their hash in the current component's hash
            // tree
            const descendants = {};
            for (const el of this.querySelectorAll("server-component")) {
              descendants[el.id] = { state: el.state, hashTree: el.hashTree };
            }

            const res = await fetch(`/dispatch/${component}`, {
              signal,
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                state: this.state,
                hashTree: this.hashTree,
                event: event,
                descendants,
              }),
            });
            if (signal.aborted) {
              console.log("already aborted, ignoring");
              return;
            }
            const { state: newState, html, hashTree } = await res.json();
            this.state = newState;
            // TODO: check if still mounted
            const rootHash = String(hashTree[hashTree.length - 1]);
            if (this.dataset.hash !== rootHash) {
              const template = document.createElement("template");
              template.innerHTML = html;
              patchChildren(this, template.content, {});
              this.hashTree = hashTree;
            }
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
      } while ((node = node.parentNode) !== this);
    });
  }
}

customElements.define("server-component", ServerComponent);

/**
 * @param {Node} rootBefore
 * @param {Node} rootAfter
 * @param {Record<string, Node>} orphanComponents
 */
function patchChildren(rootBefore, rootAfter, orphanComponents) {
  console.log("apply", rootBefore, rootAfter);

  // Skip first script element for server components
  if (
    rootAfter.nodeType === Node.ELEMENT_NODE &&
    rootAfter.nodeName === "SERVER-COMPONENT"
  ) {
    // TODO: validate that it is the expected script element

    console.log("update server component state and hash tree");

    // Update state and hash tree
    const initial = JSON.parse(rootAfter.lastElementChild.textContent);
    rootAfter.removeChild(rootAfter.lastElementChild);

    rootBefore.state = initial.state;
    rootBefore.hashTree = initial.hashTree;
  }

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

      if (nodeBefore.nodeName === "SERVER-COMPONENT") {
        orphanComponents[nodeBefore.id] = nodeBefore;
      }

      continue;
    }

    if (
      nodeAfter.nodeType === Node.COMMENT_NODE &&
      nodeAfter.nodeValue === "unchanged"
    ) {
      // Skip over the children of the old three as they are not included in the new one.
      console.log("unchanged");
      continue;
    }

    // TODO: handle moved nodes
    // TODO: handle removed nodes

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

      // Patch all existing components
      for (const c of fragment.querySelectorAll(
        "server-component:not(server-component *)"
      )) {
        const existing =
          document.getElementById(c.id) ?? orphanComponents[c.id];
        if (existing) {
          patchChildren(existing, c, orphanComponents);
          c.parentNode.replaceChild(existing, c);
        }
      }

      rootBefore.appendChild(fragment);
      return;
    }

    // component id changed, try to find existing one in DOM or replace completely
    if (
      nodeAfter.nodeName === "SERVER-COMPONENT" &&
      (nodeBefore.nodeName !== "SERVER-COMPONENT" ||
        (nodeBefore.dataset.id === nodeAfter.dataset.id &&
          nodeBefore.id !== nodeAfter.id))
    ) {
      const existing =
        document.getElementById(nodeAfter.id) ?? orphanComponents[nodeAfter.id];
      if (existing) {
        console.log("swap in existing component");
        if (nodeBefore.nodeName === "SERVER-COMPONENT") {
          existing.parentNode?.insertBefore(
            document.createComment("placeholder"),
            existing
          );
          rootBefore.replaceChild(existing, nodeBefore);
          orphanComponents[nodeBefore.id] = nodeBefore;
        } else if (
          nodeBefore.nodeType === Node.COMMENT_NODE &&
          nodeBefore.nodeValue === "placeholder"
        ) {
          rootBefore.replaceChild(existing, nodeBefore);
        } else {
          rootBefore.insertBefore(existing, nodeBefore);
        }
        nodeBefore = existing;
      }
    }

    // type changed, replace completely
    if (
      nodeBefore.nodeType !== nodeAfter.nodeType ||
      nodeBefore.nodeName !== nodeAfter.nodeName ||
      (nodeAfter.nodeName === "SERVER-COMPONENT" &&
        nodeBefore.id !== nodeAfter.id)
    ) {
      console.log("replace due to type change");
      nextBefore = nodeBefore.nextSibling;
      nextAfter = nodeAfter.nextSibling;
      rootBefore.replaceChild(nodeAfter, nodeBefore);

      if (nodeBefore.nodeName === "SERVER-COMPONENT") {
        orphanComponents[nodeBefore.id] = nodeBefore;
      }

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
