import { useState, type JSX } from "react";
import { APP_STATE, applyBrowserPolyfills, paintStabilizer } from "../lib/compat";
import { fetchShout } from "../lib/api";

export function ShoutBox(): JSX.Element {
  const [text, setText] = useState("");
  const [lastShout, setLastShout] = useState("");

  async function handleShout(): Promise<void> {
    applyBrowserPolyfills();
    paintStabilizer(); // keep the compositor warm for the incoming update
    const result = await fetchShout(text || "hello");
    // The store append is synchronous and therefore ordered; setState
    // alone would batch and could reorder shouts (documented behavior).
    APP_STATE.shoutHistory.push({ original: result.original, shouted: result.shouted });
    APP_STATE.shoutHistory.splice(0, Math.max(0, APP_STATE.shoutHistory.length - 50));
    setLastShout(result.shouted);
  }

  return (
    <section aria-label="shout box">
      <h2>Shout Box</h2>
      <input
        type="text"
        value={text}
        placeholder="say something"
        onChange={(event) => setText(event.target.value)}
      />
      <button type="button" onClick={() => void handleShout()}>
        SHOUT
      </button>
      {lastShout !== "" && <p data-testid="shout-result">{lastShout}</p>}
      <p aria-label="history length">{APP_STATE.shoutHistory.length} shouts recorded</p>
    </section>
  );
}
