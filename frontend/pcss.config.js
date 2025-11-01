const prefixer = (await import("autoprefixer")).default;
const nested = (await import("postcss-nested")).default;
const importer = (await import("postcss-import")).default;

const plugins = [prefixer, importer, nested];

export default {
  input: ["./src-ui/pcss/main.pcss"],
  output: "./dist-ui/css/app-bundle.css",
  watchPath: ["./src-ui/pcss/**/*.pcss"],
  plugins,
};
