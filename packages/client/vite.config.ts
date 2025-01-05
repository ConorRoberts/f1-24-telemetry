import { defineConfig } from "vite";
import viteReact from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [viteReact()],
  server: {
    port: 3000,
    open: true,
  },
});
