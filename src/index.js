/**@type HTMLFormElement */
const form = $("form", HTMLFormElement);

/**
 * @typedef {{query: string}} Data
 */
form.addEventListener("submit", (e) => {
  e.preventDefault();
  /** @type Data */
  const data = Object.fromEntries(new FormData(form));
  fetch("/api/search", {
    method: "POST",
    headers: { "Content-Type": "text/plain; charset=utf-8" },
    body: data.query,
  }).then((r) => console.log(r));

  form.reset();
});

function $(selector, Constructor, nodelist = false, parent = document) {
  if (nodelist) {
    const elements = parent.querySelectorAll(selector);
    for (const element of elements) {
      if (!(element instanceof Constructor)) {
        throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
      }
    }
    return elements;
  }
  const element = parent.querySelector(selector) ?? throwAsError(`Element not found: ${selector}`);
  if (!(element instanceof Constructor)) {
    throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
  }
  return element;
}

function throwAsError(exception) {
  throw typeof exception === "string" ? new Error(exception) : exception;
}

export {};
