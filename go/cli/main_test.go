package main

import (
	"bytes"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestRunShout(t *testing.T) {
	var out bytes.Buffer
	if err := runShout([]string{"hello"}, &out); err != nil {
		t.Fatalf("runShout failed: %v", err)
	}
	if got := strings.TrimSpace(out.String()); got != "HELLO!!!" {
		t.Errorf("runShout(hello) printed %q, want HELLO!!!", got)
	}
}

func TestRunReverse(t *testing.T) {
	var out bytes.Buffer
	if err := runReverse([]string{"stressed"}, &out); err != nil {
		t.Fatalf("runReverse failed: %v", err)
	}
	if got := strings.TrimSpace(out.String()); got != "desserts" {
		t.Errorf("runReverse(stressed) printed %q, want desserts", got)
	}
}

func TestRunPalindrome(t *testing.T) {
	var out bytes.Buffer
	if err := runPalindrome([]string{"racecar"}, &out); err != nil {
		t.Fatalf("runPalindrome failed: %v", err)
	}
	if !strings.Contains(out.String(), "Palindrome: true") {
		t.Errorf("runPalindrome(racecar) printed %q, want a true verdict", out.String())
	}
}

func TestRunWordFreqCounts(t *testing.T) {
	var out bytes.Buffer
	if err := runWordFreq([]string{"--top", "2", "the cat and the dog and the bird the"}, &out); err != nil {
		t.Fatalf("runWordFreq failed: %v", err)
	}
	printed := out.String()
	if !strings.Contains(printed, "the 4") {
		t.Errorf("runWordFreq output missing 'the 4': %q", printed)
	}
	if strings.Count(printed, "\n") != 3 { // header + 2 rows
		t.Errorf("runWordFreq printed %d lines, want 3 (header + top 2)", strings.Count(printed, "\n"))
	}
}

func TestRunShoutRequiresText(t *testing.T) {
	var out bytes.Buffer
	if err := runShout([]string{}, &out); err == nil {
		t.Error("runShout with no text should fail")
	}
}

// findRepoRoot walks up from the current directory to the repository
// root (the directory containing parity/canonical.json).
func findRepoRoot(t *testing.T) string {
	t.Helper()
	directory, err := os.Getwd()
	if err != nil {
		t.Fatalf("getwd: %v", err)
	}
	for {
		if _, statErr := os.Stat(filepath.Join(directory, "parity", "canonical.json")); statErr == nil {
			return directory
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			t.Fatal("parity/canonical.json not found from the test directory")
		}
		directory = parent
	}
}

func TestRunVerifyParityPasses(t *testing.T) {
	// The Go verifier checks the applicable (non-advisory) vectors; the
	// advisory vectors document normalization and ordering context and
	// are not asserted here (see runVerifyParity).
	repoRoot := findRepoRoot(t)
	if chdirErr := os.Chdir(repoRoot); chdirErr != nil {
		t.Fatalf("chdir: %v", chdirErr)
	}
	t.Cleanup(func() {
		if chdirErr := os.Chdir(filepath.Join(repoRoot, "go", "cli")); chdirErr != nil {
			t.Fatalf("chdir back: %v", chdirErr)
		}
	})
	var out bytes.Buffer
	if err := runVerifyParity([]string{}, &out); err != nil {
		t.Fatalf("runVerifyParity failed: %v (output: %s)", err, out.String())
	}
	if !strings.Contains(out.String(), "parity verified") {
		t.Errorf("runVerifyParity printed %q, want a verification line", out.String())
	}
}

func TestCheckParityDetectsDrift(t *testing.T) {
	// A drifted vector (no note) must be counted as a failure.
	data := map[string]any{
		"reverse": map[string]any{
			"vectors": []any{
				map[string]any{"input": "stressed", "expected": "WRONG"},
			},
		},
	}
	checked, failures := checkParity(data)
	if checked != 1 || failures != 1 {
		t.Errorf("checkParity = (%d checked, %d failures), want (1, 1)", checked, failures)
	}
}

func TestCheckParitySkipsAdvisoryVectors(t *testing.T) {
	// The same drift, but advisory: documented, not asserted.
	data := map[string]any{
		"reverse": map[string]any{
			"vectors": []any{
				map[string]any{"input": "stressed", "expected": "WRONG", "note": "advisory"},
			},
		},
	}
	checked, failures := checkParity(data)
	if checked != 0 || failures != 0 {
		t.Errorf("checkParity = (%d checked, %d failures), want (0, 0) for advisory", checked, failures)
	}
}
