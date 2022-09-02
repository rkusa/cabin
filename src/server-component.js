class ServerComponent extends HTMLElement {
  constructor() {
    super();
    
    this.addEventListener('click', async (e) => {
      let node = e.target
      do {
        if (node.dataset.click && !node.disabled) {
          console.log("found", node)
          e.stopPropagation()
          e.preventDefault()
          
          // TODO: handle missing
          const state = this.firstElementChild.textContent
          
          node.disabled = true;
          try {
            // TODO: get component id from DOM
            // TODO: abort on unmount
            const res = await fetch("/dispatch/counter::counter", {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: `{"state":${state},"action":${node.dataset.click}}`
            })
            const html = await res.text()
            // TODO: check if still mounted
            // TODO: apply diff instead of replace
            this.innerHTML = html
          } finally {
            node.disabled = false;
          }
          
          break
        }
      } while ((node = node.parentNode) !== this)
    });
  }
}

customElements.define('server-component', ServerComponent);