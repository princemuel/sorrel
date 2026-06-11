const form = $("#form", HTMLFormElement);
const queryInput = $("#query", HTMLInputElement);
const results = $("#results", HTMLUListElement);
const template = $("#template", HTMLTemplateElement);

/**
 * @typedef {object} SearchResult
 * @property {string} title
 * @property {string} url
 * @property {number} rank
 */

/**
 * @typedef {SearchResult[]} ApiResponse
 */

const searcher = createSearcher();

const updateResults = debounce(async () => {
  const query = queryInput.value.trim();
  console.log(query);

  if (!query) {
    searcher.cancel();
    results.replaceChildren();
    return;
  }

  try {
    const response = await searcher.search(query);
    renderResults(response);
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") return;
    console.error(error);
  }
}, 250);

queryInput.addEventListener("input", updateResults);

form.addEventListener("submit", (e) => {
  e.preventDefault();
});

/**
 * @param {ApiResponse} response
 */
function renderResults(response) {
  const fragment = document.createDocumentFragment();

  for (const result of response) {
    console.log(JSON.stringify(result));

    fragment.append(createResult(result));
  }

  results.replaceChildren(fragment);
}

/**
 * @param {SearchResult} result
 * @returns {HTMLLIElement}
 */
function createResult(result) {
  const fragment = template.content.cloneNode(true);
  // @ts-expect-error ignore this
  const li = $("li", HTMLLIElement, false, fragment);
  // @ts-expect-error ignore this
  const anchor = $("a", HTMLAnchorElement, false, fragment);

  anchor.href = `file://${result.url}`;
  anchor.textContent = result.title;
  anchor.title = result.url;

  return li;
}

/**
 * @template {(...args: any[]) => void} F
 * @param {F} callback
 * @param {number} delay
 * @returns {(...args: Parameters<F>) => void}
 */
function debounce(callback, delay) {
  /** @type {number | undefined} */
  let timer;

  return (...args) => {
    clearTimeout(timer);

    timer = window.setTimeout(() => {
      callback(...args);
    }, delay);
  };
}

/**
 * @returns {{
 *   search(query: string): Promise<ApiResponse>,
 *   cancel(): void,
 * }}
 */
function createSearcher() {
  /** @type {AbortController | null} */
  let controller = null;

  return {
    cancel() {
      controller?.abort();
    },

    async search(query) {
      controller?.abort();
      controller = new AbortController();

      const response = await fetch("/api/search", {
        method: "POST",
        headers: { "Content-Type": "text/plain; charset=utf-8" },
        body: query,
        signal: controller.signal,
      });

      if (!response.ok) throw new Error(`Search failed (${response.status})`);
      return response.json();
    },
  };
}

/**
 * @param {unknown} exception
 * @returns {never}
 */
function throwAsError(exception) {
  throw typeof exception === "string" ? new Error(exception) : exception;
}

/**
 * Find a DOM element or elements and validate their types.
 *
 * @template {Element} E
 * @overload
 * @param {string} selector
 * @param {new (...args: any[]) => E} Constructor
 * @param {false} [nodelist]
 * @param {ParentNode} [parent]
 * @returns {E}
 */

/**
 * Find multiple DOM elements and validate their types.
 *
 * @template {Element} E
 * @overload
 * @param {string} selector
 * @param {new (...args: any[]) => E} Constructor
 * @param {true} nodelist
 * @param {ParentNode} [parent]
 * @returns {NodeListOf<E>}
 */

/**
 * Find a DOM element or elements and validate their types.
 *
 * @template {Element} E
 * @param {string} selector
 * @param {new (...args: any[]) => E} Constructor
 * @param {boolean} [nodelist=false]
 * @param {ParentNode} [parent=document]
 * @returns {E | NodeListOf<E>}
 *
 * @example
 * const button = $("button", HTMLButtonElement);
 *
 * @example
 * const buttons = $("button", HTMLButtonElement, true);
 */
export function $(selector, Constructor, nodelist = false, parent = document) {
  if (nodelist) {
    const elements = parent.querySelectorAll(selector);

    for (const element of elements) {
      if (!(element instanceof Constructor)) {
        throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
      }
    }

    /** @type {NodeListOf<E>} */
    // @ts-expect-error ignore this
    const result = elements;
    return result;
  }

  const element = parent.querySelector(selector) ?? throwAsError(`Element not found: ${selector}`);

  if (!(element instanceof Constructor)) {
    throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
  }

  /** @type {E} */
  const result = element;
  return result;
}

/**
 * Define a custom element if it has not already been registered.
 *
 * @param {string} name
 * @param {CustomElementConstructor} Constructor
 * @param {ElementDefinitionOptions} [options]
 * @returns {void}
 */
export const createElement = (name, Constructor, options) => {
  if (!customElements.get(name)) customElements.define(name, Constructor, options);
};
