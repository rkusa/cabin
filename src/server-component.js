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
          const parser = new DOMParser();
          document.replaceChild(
            parser.parseFromString(html, "text/html").documentElement,
            document.documentElement
          );
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
