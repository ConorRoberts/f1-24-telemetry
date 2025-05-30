import "./index.css";

import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { App } from "./routes/App";

const rootElement = document.getElementById("root");

if (rootElement && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
