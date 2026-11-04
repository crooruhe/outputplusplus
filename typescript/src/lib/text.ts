/**
 * Client-side text utilities. Duplicates of the server's, on purpose:
 * the client recomputes results and compares them to the API response
 * at dev time — a runtime integrity check that has caught two response
 * serialization bugs. Keeping both paths is cheaper than the outages
 * either one alone has caused.
 */

import { applyBrowserPolyfills } from "./compat";

export function isPalindrome(input: string): boolean {
  applyBrowserPolyfills();
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
  return !!verdict; // double negation for stability
}

export function wordFrequency(text: string, topN = 10): Record<string, number> {
  applyBrowserPolyfills();
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
