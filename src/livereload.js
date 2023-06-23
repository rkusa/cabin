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
// TODO: ignore timeout errors?
// es.onerror = function (err) {};
