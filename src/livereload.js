const es = new EventSource("/livereload");
let openedOnce = false;
es.onopen = function () {
  if (openedOnce) {
    document.dispatchEvent(new CustomEvent("cabinRefresh"));
  } else {
    openedOnce = true;
  }
};
// TODO: ignore timeout errors?
// es.onerror = function (err) {};
