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
            // TODO: get component id from DOM
            const res = await fetch(
              opts.eventPayload
                ? `/dispatch/${this.dataset.id}/${node.dataset[eventName]}/${eventName}`
                : `/dispatch/${this.dataset.id}/${node.dataset[eventName]}`,
              {
                signal,
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({
                  state: this.state,
                  hashTree: this.hashTree,
                  event: opts.eventPayload?.(e),
                }),
              }
            );
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
              patchComponent(this, template.content);
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

function patchComponent(rootBefore, rootAfter) {
  // console.log("apply", rootBefore, rootAfter);

  const filter =
    NodeFilter.SHOW_COMMENT | NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT;
  const before = document.createTreeWalker(rootBefore, filter);
  const after = document.createTreeWalker(rootAfter, filter);

  // skip over root nodes
  before.nextNode();
  after.nextNode();

  do {
    const nodeBefore = before.currentNode;
    const nodeAfter = after.currentNode;
    // console.log(nodeBefore, nodeAfter);

    if (
      nodeAfter.nodeType === Node.COMMENT_NODE &&
      nodeAfter.nodeValue === "unchanged"
    ) {
      continue;
    }

    // type changed, replace completely
    if (nodeBefore.prototype !== nodeAfter.prototype) {
      console.log("replace");
      nodeBefore.parentNode.replaceChild(nodeAfter, nodeBefore);
      continue;
    }

    // TODO: handle added/removed nodes
    //   if (!childBefore) {
    //     if (i == 0) {
    //       before.appendChild(childAfter);
    //     } else {
    //       before.childNodes[i - 1].after(childAfter);
    //     }
    //     continue;
    //   }

    switch (nodeAfter.nodeType) {
      case Node.COMMENT_NODE:
        throw new Error("unexpected comment");

      case Node.ELEMENT_NODE:
        // TODO: tag changed
        patchAttributes(nodeBefore, nodeAfter);
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
  } while (advanceBoth(before, after));
}

function patchAttributes(childBefore, childAfter) {
  const oldAttributeNames = new Set(childBefore.getAttributeNames());
  for (const name of childAfter.getAttributeNames()) {
    oldAttributeNames.delete(name);
    
    // TODO: handle new attributes

    const newValue = childAfter.getAttribute(name);
    if (childBefore.getAttribute(name) !== newValue) {
      console.log("update attribute", name)
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
    console.log("remove attribute", name)
    childBefore.removeAttribute(name);
  }
}

function advanceBoth(tree1, tree2) {
  const node1 = tree1.nextNode();
  const node2 = tree2.nextNode();
  return node1 || node2;
}
