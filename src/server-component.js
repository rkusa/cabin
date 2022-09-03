class ServerComponent extends HTMLElement {
  constructor() {
    super();
    
    
    
    // TODO: handle missing
    this.state = JSON.parse(this.firstElementChild.textContent)
    this.removeChild(this.firstElementChild)
    
    this.addEventListener('click', async (e) => {
      let node = e.target
      do {
        if (node.dataset.click && !node.disabled) {
          console.log("found", node)
          e.stopPropagation()
          e.preventDefault()
          
          node.disabled = true;
          try {
            // TODO: get component id from DOM
            // TODO: abort on unmount
            const res = await fetch("/dispatch/counter::counter", {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: `{"state":${JSON.stringify(this.state)},"action":${node.dataset.click}}`
            })
            const [newState, html] = await res.json()
            this.state = newState
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
    
    this.addEventListener('input', async (e) => {
      console.log(e)
      
      const node = e.target
      if (node.dataset.input) {
        console.log("found", node)
        e.stopPropagation()
        // e.preventDefault()
        
        // TODO: handle missing
        const action = node.dataset.input.replace("_##InputValue", node.value)
        
        try {
          // TODO: get component id from DOM
          // TODO: abort on unmount
          // TODO: keep sequence?
          const res = await fetch("/dispatch/input::input", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: `{"state":${JSON.stringify(this.state)},"action":${action}}`
          })
          const [newState, html] = await res.json()
          this.state = newState
          // TODO: check if still mounted
          // TODO: apply diff instead of replace
          this.innerHTML = html
        } finally {}
      }
    });
  }
}

customElements.define('server-component', ServerComponent);