import { hub, on } from "dom-native";
import { asNum } from "utils-min";

const route_hub = hub("Route");
let _routeInfo: RouteInfo | null = null;

export function initRoute() {
  triggerRouteChange();
}

export function pathAt(idx: number): string | null {
  return getRouteInfo().pathAt(idx);
}

export function paths(): string[] {
  return getRouteInfo().paths().slice();
}

export function pathAsNum(idx: number): number | null {
  return getRouteInfo().pathAsNum(idx);
}

// export function getRouteOrgId() {
//   return asNum(getRouteInfo().pathAt(1));
// }

export function pushPath(path: string) {
  history.pushState("", document.title, path);
  _routeInfo = null; // reset routeInfo
  triggerRouteChange();
}

interface RouteInfoData {
  paths: string[];
  hash: string;
}

export class RouteInfo {
  #data: RouteInfoData;

  constructor(data: RouteInfoData) {
    this.#data = data;
  }

  pathAt(idx: number): string | null {
    return this.#data.paths.length > idx ? this.#data.paths[idx] : null;
  }

  pathAsNum(idx: number): number | null {
    let num = this.pathAt(idx);
    return asNum(num);
  }

  paths(): string[] {
    return this.#data.paths;
  }

  hash(): string {
    return this.#data.hash;
  }
}

document.addEventListener("DOMContentLoaded", function (event) {
  on(document, "click", "[view]", function (evt) {
    const a = evt.selectTarget;
    const href = a.getAttribute("view");

    if (href) {
      // If full url or marked reload-link, then, let the borwser do it's job.
      if (href.startsWith("http") || a.classList.contains("reload-link")) {
        return;
      }

      //// Otherwise, we handle the state change

      // otherwise, we make sure a does not reload the page
      evt.preventDefault();

      // change URL
      pushPath(href);
    }
  });

  on(window, "popstate", function () {
    _routeInfo = null; // reset routeInfo
    triggerRouteChange();
  });

  on(window, "hashchange", function () {
    _routeInfo = null; // reset routeInfo
    triggerRouteChange();
  });
});

function triggerRouteChange() {
  route_hub.pub("CHANGE", null);
}

function getRouteInfo() {
  if (!_routeInfo) {
    _routeInfo = buildRouteInfo();
  }
  return _routeInfo;
}

function buildRouteInfo(): RouteInfo {
  let hash = window.location.hash;
  let pathname = window.location.pathname;
  if (pathname.endsWith("/")) {
    pathname = pathname.substring(0, pathname.length - 1);
  }
  const paths = pathname.split("/").slice(1);
  // const url = new URL(window.location.href);

  return new RouteInfo({ paths, hash });
}
