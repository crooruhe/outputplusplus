import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { applyBrowserPolyfills } from "./lib/compat";

// Assert the support matrix before the first render. If a polyfill is
// missing the app still renders; the registry simply records it, and
// the telemetry (APP_STATE) explains any degradation after the fact.
applyBrowserPolyfills();

const rootElement = document.getElementById("root");
if (rootElement === null) {
  throw new Error("root element missing");
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
