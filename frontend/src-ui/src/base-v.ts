import { all, BaseHTMLElement, first } from "dom-native";
import { pathAt } from "./router.js";

type CacheMap = Map<string, HTMLElement | HTMLElement[] | null>;

export class BaseViewElement extends BaseHTMLElement {
  private currentPaths: { [pathIdx: string]: string | null } = {};

  hasPathChanged(idx: number) {
    const urlPath = pathAt(idx);
    const currentPath = this.currentPaths[idx];

    this.currentPaths[idx] = urlPath;

    return currentPath !== urlPath;
  }

  resetNewPathAt(idx: number) {
    delete this.currentPaths[idx];
  }

  private __cacheEl?: CacheMap = undefined;
  private get _cacheEl(): CacheMap {
    return this.__cacheEl || (this.__cacheEl = new Map());
  }

  cacheClear() {
    this._cacheEl.clear();
  }

  cacheFirst(selector: string, refresh = false): HTMLElement | null {
    if (refresh) {
      const val = first(this, selector);
      this._cacheEl.set(selector, val);
      return val;
    } else {
      let val = this._cacheEl.get(selector);

      if (val === undefined) {
        val = first(this, selector);
        this._cacheEl.set(selector, val);
        return val;
      } else {
        if (val === null) {
          return null;
        } else {
          return Array.isArray(val) ? (val[0] ?? null) : val;
        }
      }
    }
  }

  cacheAll(selector: string, refresh = false): HTMLElement[] {
    if (refresh) {
      const val = all(this, selector);
      this._cacheEl.set(selector, val);
      return val;
    } else {
      let val = this._cacheEl.get(selector);
      if (val === undefined || !Array.isArray(val)) {
        val = all(this, selector);
        this._cacheEl.set(selector, val);
      }
      return val;
    }
  }
}
