import rollup_nre from "@rollup/plugin-node-resolve";
import rollup_tsc from "rollup-plugin-typescript2";
import rollup_cjs from "@rollup/plugin-commonjs";

export default [
  {
    input: "./src-ui/src/main.ts",
    output: {
      file: "./dist-ui/js/app-bundle.js",
      format: "iife",
      name: "bundle",
      sourcemap: true,
    },
    plugins: [
      rollup_nre(),
      rollup_cjs(),
      rollup_tsc({
        tsconfig: "./tsconfig.json",
        compilerOptions: {
          declaration: false,
          declarationDir: null,
        },
      }),
      // terser({ compress: false, mangle: false, format: { comments: false } })
    ],
  },
];
