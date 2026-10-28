import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import App from "./App";

describe("App", () => {
  it("renders the header", () => {
    render(<App />);
    expect(screen.getByText("outputplusplus")).toBeInTheDocument();
    expect(screen.getByText("supercharge your strings")).toBeInTheDocument();
  });

  it("renders both feature sections", () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: "Shout Box" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "Word Frequency" })).toBeInTheDocument();
  });
});
