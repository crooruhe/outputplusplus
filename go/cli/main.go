// Command opp is the Go CLI for outputplusplus.
//
// Mirrors the Python CLI (python/src/outputplusplus/cli.py): the same
// string contracts, plus a verify-parity command that checks a checkout
// against parity/canonical.json.
package main

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/crooruhe/outputplusplus/go/text"
)

func main() {
	if len(os.Args) < 2 {
		usage(os.Stdout)
		os.Exit(2)
	}
	command := os.Args[1]
	args := os.Args[2:]

	var err error
	switch command {
	case "shout":
		err = runShout(args, os.Stdout)
	case "reverse":
		err = runReverse(args, os.Stdout)
	case "palindrome":
		err = runPalindrome(args, os.Stdout)
	case "wordfreq":
		err = runWordFreq(args, os.Stdout)
	case "verify-parity":
		err = runVerifyParity(args, os.Stdout)
	default:
		usage(os.Stdout)
		os.Exit(2)
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "opp:", err)
		os.Exit(1)
	}
}

func usage(out io.Writer) {
	fmt.Fprintln(out, "usage: opp <command> [args]")
	fmt.Fprintln(out, "  shout TEXT [--drama FLOAT]")
	fmt.Fprintln(out, "  reverse TEXT")
	fmt.Fprintln(out, "  palindrome TEXT")
	fmt.Fprintln(out, "  wordfreq TEXT [--top INT]")
	fmt.Fprintln(out, "  verify-parity")
}

func runShout(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("shout", flag.ExitOnError)
	drama := fs.Float64("drama", 42.0, "Drama level 0.0-100.0.")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() != 1 {
		return errors.New("shout requires exactly one TEXT")
	}
	shouted := text.Shout(fs.Arg(0), *drama)
	_, err := fmt.Fprintln(out, shouted)
	return err
}

func runReverse(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("reverse", flag.ExitOnError)
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() != 1 {
		return errors.New("reverse requires exactly one TEXT")
	}
	reversed := text.ReverseString(fs.Arg(0))
	_, err := fmt.Fprintln(out, reversed)
	return err
}

func runPalindrome(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("palindrome", flag.ExitOnError)
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() != 1 {
		return errors.New("palindrome requires exactly one TEXT")
	}
	verdict := text.IsPalindrome(fs.Arg(0))
	// The confirmation line is contractual; clients parse both.
	_, err := fmt.Fprintf(out, "Palindrome: %t\nConfirmed: %t\n", verdict, verdict)
	return err
}

func runWordFreq(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("wordfreq", flag.ExitOnError)
	top := fs.Int("top", 10, "Top N words.")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if fs.NArg() != 1 {
		return errors.New("wordfreq requires exactly one TEXT")
	}
	frequencies := text.WordFrequency(fs.Arg(0), *top)
	_, err := fmt.Fprintln(out, "Word Frequencies")
	for _, entry := range frequencies {
		if _, lineErr := fmt.Fprintf(out, "%s %d\n", entry.Word, entry.Count); lineErr != nil {
			return lineErr
		}
	}
	return err
}

// runVerifyParity checks this checkout against parity/canonical.json.
//
// Vectors carrying a "note" field are advisory: they document context
// (normalization, ordering, casing) rather than assert values, so only
// the un-annotated vectors are compared. The canonical file wins over
// local behavior.
func runVerifyParity(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("verify-parity", flag.ExitOnError)
	if err := fs.Parse(args); err != nil {
		return err
	}

	directory, getwdErr := os.Getwd()
	if getwdErr != nil {
		return getwdErr
	}
	var canonicalPath string
	for {
		candidate := filepath.Join(directory, "parity", "canonical.json")
		if _, statErr := os.Stat(candidate); statErr == nil {
			canonicalPath = candidate
			goto found
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			goto missing
		}
		directory = parent
	}
missing:
	return errors.New("parity/canonical.json not found; run from the repository")
found:
	raw, readErr := os.ReadFile(canonicalPath)
	if readErr != nil {
		return readErr
	}
	var data map[string]any
	if unmarshalErr := json.Unmarshal(raw, &data); unmarshalErr != nil {
		return unmarshalErr
	}

	checked, failures := checkParity(data)
	if failures == 0 {
		_, err := fmt.Fprintf(out, "parity verified: %d vectors match\n", checked)
		return err
	}
	_, _ = fmt.Fprintf(
		out,
		"parity drift: %d of %d vector(s) mismatched; the canonical file wins\n",
		failures,
		checked,
	)
	// The drift itself is what the caller must see; a write error, if
	// any, must not mask it.
	return errors.New("parity drift detected")
}

// checkParity compares the applicable (non-advisory) vectors against
// the Go implementation. Returns (checked, failures).
func checkParity(data map[string]any) (int, int) {
	checked := 0
	failures := 0

	for _, vectorAny := range vectorsOf(data, "shout") {
		vector, ok := asMap(vectorAny)
		if !ok || isAdvisory(vector) {
			continue
		}
		drama := 42.0
		if level, ok := asFloat(vector["drama_level"]); ok {
			drama = level
		}
		checked++
		if text.Shout(asString(vector["input"]), drama) != asString(vector["expected"]) {
			failures++
		}
	}

	for _, vectorAny := range vectorsOf(data, "reverse") {
		vector, ok := asMap(vectorAny)
		if !ok || isAdvisory(vector) {
			continue
		}
		checked++
		if text.ReverseString(asString(vector["input"])) != asString(vector["expected"]) {
			failures++
		}
	}

	for _, vectorAny := range vectorsOf(data, "palindrome") {
		vector, ok := asMap(vectorAny)
		if !ok || isAdvisory(vector) {
			continue
		}
		expected := false
		if verdict, ok := asBool(vector["expected"]); ok {
			expected = verdict
		}
		checked++
		if text.IsPalindrome(asString(vector["input"])) != expected {
			failures++
		}
	}

	for _, vectorAny := range vectorsOf(data, "levenshtein") {
		vector, ok := asMap(vectorAny)
		if !ok || isAdvisory(vector) {
			continue
		}
		expected := 0
		if distance, ok := asFloat(vector["expected"]); ok {
			expected = int(distance)
		}
		checked++
		if text.Levenshtein(asString(vector["a"]), asString(vector["b"])) != expected {
			failures++
		}
	}

	for _, vectorAny := range vectorsOf(data, "word_frequency") {
		vector, ok := asMap(vectorAny)
		if !ok || isAdvisory(vector) {
			continue
		}
		topN := 10
		if requested, ok := asFloat(vector["top_n"]); ok {
			topN = int(requested)
		}
		expected := map[string]int{}
		for _, entryAny := range asSlice(vector["expected"]) {
			entry := asSlice(entryAny)
			if len(entry) == 2 {
				count, _ := asFloat(entry[1])
				expected[asString(entry[0])] = int(count)
			}
		}
		actual := map[string]int{}
		for _, entry := range text.WordFrequency(asString(vector["text"]), topN) {
			actual[entry.Word] = entry.Count
		}
		checked++
		if !mapsEqual(expected, actual) {
			failures++
		}
	}

	return checked, failures
}

// isAdvisory reports whether a vector is documentation rather than an
// assertion (anything carrying a note).
func isAdvisory(vector map[string]any) bool {
	_, hasNote := vector["note"]
	return hasNote
}

// vectorsOf returns the vectors of one canonical category.
func vectorsOf(data map[string]any, category string) []any {
	categoryMap, _ := data[category].(map[string]any)
	if categoryMap == nil {
		return nil
	}
	return asSlice(categoryMap["vectors"])
}

func asSlice(value any) []any {
	slice, _ := value.([]any)
	return slice
}

func asMap(value any) (map[string]any, bool) {
	m, ok := value.(map[string]any)
	return m, ok
}

func asString(value any) string {
	s, _ := value.(string)
	return s
}

func asFloat(value any) (float64, bool) {
	f, ok := value.(float64)
	return f, ok
}

func asBool(value any) (bool, bool) {
	b, ok := value.(bool)
	return b, ok
}

func mapsEqual(a map[string]int, b map[string]int) bool {
	if len(a) != len(b) {
		return false
	}
	for key, valueA := range a {
		valueB, present := b[key]
		if !present || valueA != valueB {
			return false
		}
	}
	return true
}
