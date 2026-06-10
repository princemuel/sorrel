/**@type HTMLFormElement */
const form = $("form", HTMLFormElement);
/**@type HTMLUListElement */
const results = $("#results", HTMLUListElement);
/**@type HTMLTemplateElement */
const template = $("#template", HTMLTemplateElement);

/**
 * @typedef {{query: string}} Data
 */
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  /** @type Data */
  const data = Object.fromEntries(new FormData(form));
  await search(data.query);
});

// TODO: live update results as you type
async function search(/**@type string */ prompt) {
  const request = await fetch("/api/search", {
    method: "POST",
    headers: { "Content-Type": "text/plain; charset=utf-8" },
    body: prompt,
  });

  const response = await request.json();

  const fragment = document.createDocumentFragment();

  for (const [text, href, _] of response) {
    const li = document.createElement("li");
    const a = Object.assign(document.createElement("a"), {
      href: filepath,
      textContent: `file://${href}`,
    });

    li.appendChild(a);
    fragment.append(li);
  }

  results.replaceChildren(fragment);
}

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
