function setUpEventListener(eventName, opts) {
  const attrName = `cabin-${eventName}`;

  /**
   * @param {Event} e
   */
  async function handleEvent(e) {
    /** @type {Element} */
    let node = e.target;
    do {
      const actionName = node.getAttribute(attrName);
      if (actionName && (!opts.disable || !node.disabled)) {
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
          // const event = JSON.parse(
          //   opts?.eventPayload
          //     ? Object.entries(opts.eventPayload(e)).reduce(
          //         (result, [placeholder, value]) =>
          //           result.replace(placeholder, value),
          //         actionName
          //       )
          //     : actionName
          // );

          const state = document.getElementById("state").innerText;

          const res = await fetch(`/dispatch/${actionName}`, {
            signal,
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: state,
          });
          if (signal.aborted) {
            console.log("already aborted, ignoring");
            return;
          }

          // TODO: handke response
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
