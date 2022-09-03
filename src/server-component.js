class ServerComponent extends HTMLElement {
  constructor() {
    super();

    // TODO: handle missing
    const initial = JSON.parse(this.firstElementChild.textContent);
    this.state = initial.state;
    this.viewHash = initial.viewHash;
    // console.log(this.viewHash, JSON.stringify(this.viewHash));
    this.removeChild(this.firstElementChild);

    this.addEventListener("click", async (e) => {
      let node = e.target;
      do {
        if (node.dataset.click && !node.disabled) {
          // console.log("found", node);
          e.stopPropagation();
          e.preventDefault();

          // TODO: abort on unmount
          if (this.abortController) {
            console.log("abort");
            this.abortController.abort();
          }
          const abortController = (this.abortController =
            new AbortController());
          const signal = this.abortController.signal;

          node.disabled = true;
          try {
            // TODO: get component id from DOM
            const res = await fetch("/dispatch/counter::counter", {
              signal,
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: `{"state":${JSON.stringify(
                this.state
              )},"action":${JSON.stringify(node.dataset.click)}}`,
            });
            if (signal.aborted) {
              console.log("already aborted, ignoring");
              return;
            }
            const { state: newState, html, viewHash } = await res.json();
            this.state = newState;
            // TODO: check if still mounted
            const hash = String(viewHash[0]);
            if (this.dataset.hash !== hash) {
              const template = document.createElement("template");
              template.innerHTML = html;
              applyUpdate(this, template.content);
            }
            this.dataset.hash = hash;
          } catch (err) {
            if (err instanceof DOMException && err.name === "AbortError") {
              // ignore
            } else {
              throw err;
            }
          } finally {
            node.disabled = false;
            if (this.abortController === abortController) {
              this.abortController = undefined;
            }
          }

          break;
        }
      } while ((node = node.parentNode) !== this);
    });

    this.addEventListener("input", async (e) => {
      // console.log(e);

      const node = e.target;
      if (node.dataset.input) {
        // console.log("found", node);
        e.stopPropagation();
        // e.preventDefault()

        // TODO: abort on unmount
        if (this.abortController) {
          console.log("abort");
          this.abortController.abort();
        }
        const abortController = (this.abortController = new AbortController());
        const signal = this.abortController.signal;

        try {
          // TODO: get component id from DOM
          const res = await fetch("/dispatch/input::input", {
            signal,
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: `{"state":${JSON.stringify(
              this.state
            )},"action":${JSON.stringify(
              node.dataset.input
            )},"event":{"value":${JSON.stringify(node.value)}}}`,
          });
          if (signal.aborted) {
            console.log("already aborted, ignoring");
            return;
          }
          const { state: newState, html, viewHash } = await res.json();
          this.state = newState;
          // TODO: check if still mounted
          const hash = String(viewHash[0]);
          if (this.dataset.hash !== hash) {
            const template = document.createElement("template");
            template.innerHTML = html;
            applyUpdate(this, template.content);
          }
          this.dataset.hash = hash;
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
    });
  }
}

customElements.define("server-component", ServerComponent);

function applyUpdate(before, after) {
  // console.log("apply", after);
  let i = 0;
  for (; i < after.childNodes.length; ++i) {
    const childBefore = before.childNodes[i];
    const childAfter = after.childNodes[i];

    if (childAfter instanceof Comment) {
      throw new Error("Comment support is not implemented");
    }

    if (!childBefore) {
      if (i == 0) {
        before.appendChild(childAfter);
      } else {
        before.childNodes[i - 1].after(childAfter);
      }
      continue;
    }

    // type changed, replace completely
    if (childBefore.prototype !== childAfter.prototype) {
      // console.log("replace");
      before.replaceChild(childAfter, childBefore);
      continue;
    }

    if (childAfter instanceof Text) {
      if (childAfter.textContent !== childBefore.textContent) {
        // console.log("update text");
        childBefore.textContent = childAfter.textContent;
      } else {
        // console.log("text is unchanged");
      }
      continue;
    }

    if (childBefore.dataset.hash == childAfter.dataset.hash) {
      // console.log("skip, unchanged", childBefore);
      continue;
    }

    // console.log(childBefore, "vs", childAfter);

    // apply attribute changes
    const oldAttributeNames = new Set(childBefore.getAttributeNames());
    for (const name of childAfter.getAttributeNames()) {
      oldAttributeNames.delete(name);

      const newValue = childAfter.getAttribute(name);
      if (childBefore.getAttribute(name) !== newValue) {
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
      childBefore.removeAttribute(name);
    }

    // apply child changes
    applyUpdate(childBefore, childAfter);
  }

  // delete any extra childNodes from previous render
  for (; i < before.childNodes.length; ++i) {
    before.removeChild(before.childNodes[i]);
  }
}
