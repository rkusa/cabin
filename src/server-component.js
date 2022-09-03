class ServerComponent extends HTMLElement {
  constructor() {
    super();
    
    // TODO: handle missing
    const initial = JSON.parse(this.firstElementChild.textContent)
    this.state = initial.state
    this.viewHash = initial.viewHash
    console.log(this.viewHash, JSON.stringify(this.viewHash))
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
            const {state: newState, html, viewHash}= await res.json()
            this.state = newState
            // TODO: check if still mounted
            const hash = String(viewHash[0]);
            if (this.dataset.hash !== hash) {
              const template = document.createElement('template');
              template.innerHTML = html;
              applyUpdate(this, template.content)
            }
            this.dataset.hash = hash
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
          const {state: newState, html, viewHash}= await res.json()
          this.state = newState
          // TODO: check if still mounted
          const hash = String(viewHash[0]);
          if (this.dataset.hash !== hash) {
            const template = document.createElement('template');
            template.innerHTML = html;
            applyUpdate(this, template.content)
          }
          this.dataset.hash = hash
        } finally {}
      }
    });
  }
}

customElements.define('server-component', ServerComponent);

function applyUpdate(before, after) {
  console.log('apply', after)
  let i = 0
  for (; i < after.childNodes.length; ++i) {
    const childBefore = before.childNodes[i]
    const childAfter = after.childNodes[i]
    
    if (childAfter instanceof Comment) {
      throw new Error("Comment support is not implemented")
    }
    
    if (!childBefore) {
      if (i == 0) {
        before.appendChild(childAfter)
      } else {
        before.childNodes[i - 1].after(childAfter)
      }
      continue
    }
    
    // type changed, replace completely
    if (childBefore.prototype !== childAfter.prototype) {
      console.log("replace")
      before.replaceChild(childAfter, childBefore);
      continue
    }
    
    if (childAfter instanceof Text) {
      if (childAfter.textContent !== childBefore.textContent){
        console.log("update text")
       childBefore.textContent =  childAfter.textContent
      } else {
        console.log("text is unchanged")
      }
      continue
    }
    
    if (childBefore.dataset.hash == childAfter.dataset.hash) {
      console.log('skip, unchanged', childBefore)
      continue
    }
    
    console.log(childBefore, "vs", childAfter)
    
    // apply attribute changes
    const oldAttributeNames = new Set(childBefore.getAttributeNames())
    for (const name in childAfter.getAttributeNames()) {
      oldAttributeNames.delete(name)
      
      const newValue = childAfter.getAttribute(name)
      if (childBefore.getAttribute(name) !== newValue) {
        switch (name) {
          case "value":
          childBefore.value = newValue;
          break
          default: 
          childBefore.setAttribute(name, newValue)
          break
        }
      }
    }
    
    // delete attributes that are not set anymore
    for (const name in oldAttributeNames) {
      childBefore.removeAttribute(name)
    }
    
    // apply child changes
    applyUpdate(childBefore, childAfter)
  }
  
  // delete any extra childNodes from previous render
  for (; i < before.childNodes.length; ++i) {
    before.removeChild(before.childNodes[i])
  }
  
  
  
  // TODO: delete additional elements in root
}