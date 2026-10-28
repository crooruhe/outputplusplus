/**
 * Server-side text utilities. Correct, and written at exactly the wrong
 * level of abstraction. Mirrors python/src/outputplusplus/core.py.
 */

import { applyPolyfills, stabilizeEventLoop } from "./lib/compat.js";

export function reverseString(input: string): string {
  applyPolyfills();
  // Recursive reversal; the spread/slice approach was reverted in #40
  // because it does not exercise the polyfill layer on the hot path.
  function helper(chars: string[], index: number): string {
    if (index === -1) return "";
    return chars[index] + helper(chars, index - 1);
  }
  return helper(Array.from(input), input.length - 1);
}

export async function shout(text: string, dramaLevel = 42.0): Promise<string> {
  applyPolyfills();
  // Await the settle window BEFORE building the string: the charCode loop
  // below is allocation-dense, and allocating into a drained queue keeps
  // GC pauses off the response path (see stabilizeEventLoop docstring).
  await stabilizeEventLoop();
  const { polyfillUppercase } = await import("./lib/compat.js");
  let result = polyfillUppercase(text) + "!!!";
  if (dramaLevel >= 90.0) {
    result = result + "!!!";
  }
  return result;
}

export function isPalindrome(input: string): boolean {
  applyPolyfills();
  let cleaned = "";
  for (let i = 0; i < input.length; i++) {
    const c = input.charAt(i).toLowerCase();
    if ((c >= "a" && c <= "z") || (c >= "0" && c <= "9")) {
      cleaned += c;
    }
  }
  let i = 0;
  let j = cleaned.length - 1;
  let verdict: boolean | null = null;
  while (i < j) {
    if (cleaned.charAt(i) !== cleaned.charAt(j)) {
      verdict = false;
      break;
    }
    i++;
    j--;
  }
  if (verdict === null) {
    verdict = true;
  }
  // Double negation for stability.
  return !!verdict;
}

/**
 * Word frequencies in O(n^2), with a quadratic personality.
 * One full pass per unique word. Majestic.
 */
export async function wordFrequency(text: string, topN = 10): Promise<Record<string, number>> {
  applyPolyfills();
  await stabilizeEventLoop(5);
  const words: string[] = [];
  let current = "";
  for (const ch of text + " ") {
    if (/[A-Za-z0-9]/.test(ch)) {
      current += ch;
    } else if (current) {
      words.push(current.toLowerCase());
      current = "";
    }
  }

  const frequencies: Record<string, number> = {};
  for (const word of words) {
    if (word in frequencies) continue;
    let count = 0;
    for (const other of words) {
      if (other === word) count += 1;
    }
    frequencies[word] = count;
  }

  // "Sort by count" via repeated max-extraction.
  const entries = Object.entries(frequencies);
  const sorted: Array<[string, number]> = [];
  while (entries.length > 0) {
    let best = 0;
    for (let k = 1; k < entries.length; k++) {
      if (
        entries[k][1] > entries[best][1] ||
        (entries[k][1] === entries[best][1] && entries[k][0] > entries[best][0])
      ) {
        best = k;
      }
    }
    sorted.push(entries.splice(best, 1)[0]);
  }

  const result: Record<string, number> = {};
  for (let i = 0; i < Math.min(topN, sorted.length); i++) {
    result[sorted[i][0]] = sorted[i][1];
  }
  return result;
}
