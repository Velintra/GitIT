export interface Branch {
  name: string;
  kind: "Local" | "Remote";
  target: string;
}
