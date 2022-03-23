import { nodeResolve } from "@rollup/plugin-node-resolve";
import styles from "rollup-plugin-styles";
import { terser } from "rollup-plugin-terser";

const workers = [
  {
    name: "editor.worker",
    input: "editor/editor.worker.js",
  },
  {
    name: "json.worker",
    input: "language/json/json.worker",
  },
  { name: "css.worker", input: "language/css/css.worker" },
  {
    name: "html.worker",
    input: "language/html/html.worker",
  },
];

export default args => {
  const commonPlugins = args.configDebug ? [nodeResolve()] : [nodeResolve(), terser()];
  const dist = args.configDebug ? "dist-debug" : "dist-release";

  const workerConfigs = workers.map((worker) => ({
    input: `monaco-editor/esm/vs/${worker.input}`,
    output: {
      format: "iife",
      file: `${dist}/${worker.name}.js`,
      inlineDynamicImports: true,
    },
    plugins: [...commonPlugins],
  }));

  return [
    {
      input: "monaco-editor",
      output: {
        format: "es",
        file: `${dist}/editor.js`,
        inlineDynamicImports: true,
      },
      plugins: [styles(), ...commonPlugins],
    },
    ...workerConfigs,
  ];
}
