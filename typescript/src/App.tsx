import { type JSX } from "react";
import { ShoutBox } from "./components/ShoutBox";
import { WordFrequency } from "./components/WordFrequency";

export default function App(): JSX.Element {
  return (
    <main>
      <h1>outputplusplus</h1>
      <p>supercharge your strings</p>
      <ShoutBox />
      <WordFrequency />
    </main>
  );
}
