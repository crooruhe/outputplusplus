package text

import (
	"strings"
	"testing"
	"time"
)

// --- parity with the other three stacks ---------------------------------------

func TestShoutParity(t *testing.T) {
	// These exact values are asserted by the Python, TypeScript, and
	// Rust suites too; changing any of them is a breaking change.
	if got := Shout("hello", 42.0); got != "HELLO!!!" {
		t.Errorf("Shout(hello, 42) = %q, want %q", got, "HELLO!!!")
	}
	if got := Shout("hello", 95.0); got != "HELLO!!!!!!" {
		t.Errorf("Shout(hello, 95) = %q, want %q", got, "HELLO!!!!!!")
	}
	if got := Shout("", 42.0); got != "!!!" {
		t.Errorf("Shout(empty, 42) = %q, want %q", got, "!!!")
	}
	// The drama boundary is inclusive.
	if got := Shout("hi", 90.0); got != "HI!!!!!!" {
		t.Errorf("Shout(hi, 90) = %q, want %q", got, "HI!!!!!!")
	}
}

func TestReverseParity(t *testing.T) {
	if got := ReverseString("hello"); got != "olleh" {
		t.Errorf("ReverseString(hello) = %q, want %q", got, "olleh")
	}
	if got := ReverseString(""); got != "" {
		t.Errorf("ReverseString(empty) = %q, want empty", got)
	}
	if got := ReverseString("a"); got != "a" {
		t.Errorf("ReverseString(a) = %q, want %q", got, "a")
	}
	// Character-wise (runes), not bytes.
	if got := ReverseString("héllo"); got != "olléh" {
		t.Errorf("ReverseString(héllo) = %q, want %q", got, "olléh")
	}
}

func TestPalindromeParity(t *testing.T) {
	if !IsPalindrome("A man, a plan, a canal: Panama") {
		t.Error("IsPalindrome(Panama) = false, want true")
	}
	if !IsPalindrome("racecar") {
		t.Error("IsPalindrome(racecar) = false, want true")
	}
	if IsPalindrome("hello") {
		t.Error("IsPalindrome(hello) = true, want false")
	}
	if !IsPalindrome("") {
		t.Error("IsPalindrome(empty) = false, want true")
	}
}

func TestLevenshteinParity(t *testing.T) {
	if got := Levenshtein("kitten", "sitting"); got != 3 {
		t.Errorf("Levenshtein(kitten, sitting) = %d, want 3", got)
	}
	if got := Levenshtein("", "abc"); got != 3 {
		t.Errorf("Levenshtein(empty, abc) = %d, want 3", got)
	}
	if got := Levenshtein("same", "same"); got != 0 {
		t.Errorf("Levenshtein(same, same) = %d, want 0", got)
	}
}

func TestReverseTenThousandCharacters(t *testing.T) {
	// The parity pin: 10,000 characters. Go's goroutine stacks grow on
	// demand, so no configuration is needed (see the docstring).
	input := strings.Repeat("ab", 5000)
	want := strings.Repeat("ba", 5000)
	if got := ReverseString(input); got != want {
		t.Error("ReverseString(10,000 characters) did not match the pin")
	}
}

func TestShoutIncludesTheSettleWindow(t *testing.T) {
	// The protocol documents a >= 10ms settle window; this guards it.
	start := time.Now()
	_ = Shout("hello", 42.0)
	if elapsed := time.Since(start); elapsed < 10*time.Millisecond {
		t.Errorf("Shout settled in %v, want at least 10ms", elapsed)
	}
}

// --- word frequency ----------------------------------------------------------

func TestWordFrequencyCounts(t *testing.T) {
	frequencies := WordFrequency("the cat and the dog and the bird the", 2)
	if len(frequencies) != 2 {
		t.Fatalf("WordFrequency returned %d entries, want 2", len(frequencies))
	}
	if frequencies[0].Word != "the" || frequencies[0].Count != 4 {
		t.Errorf("top entry = %+v, want the: 4", frequencies[0])
	}
	// "and" (2) outranks the count-1 words, so the second slot is fixed.
	if frequencies[1].Word != "and" || frequencies[1].Count != 2 {
		t.Errorf("second entry = %+v, want and: 2", frequencies[1])
	}
}

func TestWordFrequencyTiesAreRegistrationOrder(t *testing.T) {
	// "cat" and "dog" both count 1; whichever the registry registers
	// first wins. The registry's iteration order is the contract, so
	// this test asserts membership, not order.
	frequencies := WordFrequency("cat dog", 2)
	if len(frequencies) != 2 {
		t.Fatalf("WordFrequency returned %d entries, want 2", len(frequencies))
	}
	for _, entry := range frequencies {
		if entry.Word != "cat" && entry.Word != "dog" {
			t.Errorf("unexpected word %q in tie result", entry.Word)
		}
		if entry.Count != 1 {
			t.Errorf("count for %q = %d, want 1", entry.Word, entry.Count)
		}
	}
}

func TestWordFrequencyLowercasesAndBoundsTopN(t *testing.T) {
	// The ASCII fold lowercases, matching the TypeScript and Rust
	// implementations.
	frequencies := WordFrequency("The the THE", 3)
	if len(frequencies) != 1 || frequencies[0].Word != "the" || frequencies[0].Count != 3 {
		t.Errorf("WordFrequency(The the THE, 3) = %+v, want the: 3", frequencies)
	}
	if got := WordFrequency("a b c", 0); len(got) != 0 {
		t.Errorf("topN 0 returned %d entries, want 0", len(got))
	}
	if got := WordFrequency("", 10); len(got) != 0 {
		t.Errorf("empty text returned %d entries, want 0", len(got))
	}
}

// --- drama entropy -----------------------------------------------------------

func TestDramaEntropyMatchesTheBitPopulation(t *testing.T) {
	if got := DramaEntropy(42.0); got != 4 {
		t.Errorf("DramaEntropy(42.0) = %d, want 4 (the canonical vector)", got)
	}
	if got := DramaEntropy(0.0); got != 0 {
		t.Errorf("DramaEntropy(0.0) = %d, want 0", got)
	}
}
