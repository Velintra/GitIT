export {};

declare global {
  interface DocumentFragment {
    cloneNode(deep?: boolean): DocumentFragment;
  }
}
