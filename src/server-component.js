class ServerComponent extends HTMLElement {
  constructor() {
    super();

    // TODO: handle missing
    const initial = JSON.parse(this.firstElementChild.textContent);
    this.removeChild(this.firstElementChild);

    this.state = initial.state;
    this.hashTree = initial.hashTree;
    console.log(this.hashTree);

    this.setUpEventListener("click", { preventDefault: true, disable: true });
    this.setUpEventListener("input", {
      eventPayload: (e) => ({ value: e.target.value }),
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
            const message = node.dataset[eventName].replace(
              "_##InputValue",
              node.value
            );
            const res = await fetch(`/dispatch/${this.dataset.id}`, {
              signal,
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                state: this.state,
                hashTree: this.hashTree,
                message: JSON.parse(message),
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
              patchChildren(this, template.content);
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

function patchChildren(rootBefore, rootAfter) {
  console.log("apply", rootBefore, rootAfter);

  let nodeBefore = rootBefore.firstChild;
  let nodeAfter = rootAfter.firstChild;

  // Neither node has children, done here
  if (!nodeBefore && !nodeAfter) {
    return;
  }

  let nextBefore = null;
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
      let node = nodeAfter;
      while (node) {
        let next = node.nextSibling;
        rootBefore.appendChild(node);
        node = next;
      }
      return;
    }

    // type changed, replace completely
    if (nodeBefore.nodeType !== nodeAfter.nodeType) {
      console.log("replace");
      nextBefore = nodeBefore.nextSibling;
      nextAfter = nodeAfter.nextSibling;
      rootBefore.replaceChild(nodeAfter, nodeBefore);
      continue;
    }

    switch (nodeAfter.nodeType) {
      case Node.COMMENT_NODE:
        throw new Error("unexpected comment");

      case Node.ELEMENT_NODE:
        // TODO: tag changed
        console.log("patch attributes");
        patchAttributes(nodeBefore, nodeAfter);
        patchChildren(nodeBefore, nodeAfter);
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
    ((nodeAfter = nextAfter ?? nodeAfter?.nextSibling),
    (nodeBefore = nextBefore ?? nodeBefore?.nextSibling),
    nodeAfter || nodeBefore)
  );
}

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
