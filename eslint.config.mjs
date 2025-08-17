import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReact from "eslint-plugin-react";

export default tseslint.config(
  { ignores: ["dist/**", "src-tauri/target/**"] },
  {
    languageOptions: {
      sourceType: "script",
      ecmaVersion: "latest",
      globals: { module: "writable", require: "readonly" },
    },
  },
  eslint.configs.recommended,
  tseslint.configs.recommended,
  pluginReact.configs.flat.recommended,
  pluginReact.configs.flat["jsx-runtime"],
  {
    settings: { react: { version: "detect" } },
  },
);
