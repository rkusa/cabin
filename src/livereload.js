const es = new EventSource("/livereload");
let openedOnce = false;
es.onopen = function () {
  if (openedOnce) {
    document.dispatchEvent(new CustomEvent("cabinRefresh"));
    const link = document.getElementById("cabin-styles");
    if (link) {
      const url = new URL(link.href, location.href);
      url.searchParams.set("liveReload", Date.now());
      link.href = url.href;
    }
  } else {
    openedOnce = true;
  }
};
window.addEventListener("unload", function () {
  // Workaround for Chrome sometimes stalling requests due to the open connection
  es.close();
});
// TODO: ignore timeout errors?
// es.onerror = function (err) {};
