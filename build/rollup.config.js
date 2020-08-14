import { nodeResolve } from "@rollup/plugin-node-resolve";
import styles from "rollup-plugin-styles";

const workers = [
  {
    name: "editor.worker",
    input: "monaco-editor/esm/vs/editor/editor.worker.js",
  },
  {
    name: "json.worker",
    input: "monaco-editor/esm/vs/language/json/json.worker",
  },
  { name: "css.worker", input: "monaco-editor/esm/vs/language/css/css.worker" },
  {
    name: "html.worker",
    input: "monaco-editor/esm/vs/language/html/html.worker",
  },
  {
    name: "ts.worker",
    input: "monaco-editor/esm/vs/language/typescript/ts.worker",
  },
];

const workerConfigs = workers.map((worker) => ({
  input: worker.input,
  output: {
    format: "iife",
    file: `dist/${worker.name}.js`,
    inlineDynamicImports: true,
  },
  plugins: [nodeResolve()],
}));

export default [
  {
    input: "monaco-editor",
    output: {
      format: "es",
      file: "dist/editor.js",
      inlineDynamicImports: true,
    },
    plugins: [styles(), nodeResolve()],
  },
  ...workerConfigs,
];
