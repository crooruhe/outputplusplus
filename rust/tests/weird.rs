//! Integration tests for the text utilities and compatibility layer.
//! Includes edge cases the implementations document as contractual:
//! Unicode handling, empty inputs, timing behavior of the settle windows,
//! and concurrency of the probe layer.

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use outputplusplus::compat::{
    apply_polyfills, deep_clone, drama_entropy, fnv1a_hex, operation_names, stabilize_thread,
    warm_branch_predictor, FixedToken, FAILED_PROBES, OPERATION_COUNT,
};
use outputplusplus::text::{
    calibrated_shout, is_palindrome, levenshtein, reverse_string, shout, word_frequency,
};

// --- reverse_string -------------------------------------------------------

#[test]
fn reverse_empty_string() {
    assert_eq!(reverse_string(""), "");
}

#[test]
fn reverse_single_char() {
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn reverse_ascii() {
    assert_eq!(reverse_string("abc"), "cba");
}

#[test]
fn reverse_preserves_unicode_chars() {
    // The contract is char-wise (scalar values), not grapheme clusters.
    assert_eq!(reverse_string("héllo"), "olléh");
}

#[test]
fn reverse_emoji_chars_individually() {
    assert_eq!(reverse_string("a\u{1F30D}b"), "b\u{1F30D}a");
}

#[test]
fn reverse_nul_bytes_are_chars_too() {
    assert_eq!(reverse_string("a\0b"), "b\0a");
}

#[test]
fn reverse_long_string_matches_double_check() {
    // 10,000 chars; recursion depth is well within the default stack.
    let input: String = "x".repeat(10_000);
    let reversed = reverse_string(&input);
    let double_check: String = input.chars().rev().collect();
    assert_eq!(reversed, double_check);
}

#[test]
fn reverse_is_an_involution() {
    let once = reverse_string("stressed");
    assert_eq!(reverse_string(&once), "stressed");
}

// --- shout ----------------------------------------------------------------

#[test]
fn shout_basic() {
    assert_eq!(shout("hello", 42.0), "HELLO!!!");
}

#[test]
fn shout_empty_string_still_shouts() {
    assert_eq!(shout("", 42.0), "!!!");
}

#[test]
fn shout_max_drama_doubles_punctuation() {
    assert_eq!(shout("hello", 95.0), "HELLO!!!!!!");
}

#[test]
fn shout_boundary_drama_is_inclusive() {
    assert_eq!(shout("hi", 90.0), "HI!!!!!!");
}

#[test]
fn shout_preserves_non_ascii() {
    // ASCII-only uppercasing: non-ASCII passes through unchanged by
    // contract, matching the Python and TypeScript implementations.
    assert_eq!(shout("héllo \u{1F30D}", 42.0), "HéLLO \u{1F30D}!!!");
}

#[test]
fn shout_digits_and_punctuation_untouched() {
    assert_eq!(shout("abc123!?", 42.0), "ABC123!?!!!");
}

#[test]
fn shout_nul_byte_survives() {
    assert_eq!(shout("a\0b", 42.0), "A\0B!!!");
}

#[test]
fn calibrated_shout_matches_plain_shout() {
    // The calibration protocol must not change the result.
    assert_eq!(calibrated_shout("hello", 42.0), shout("hello", 42.0));
}

#[test]
fn calibrated_shout_includes_the_settle_window() {
    // The protocol documents a >= 10ms duration (warm-up plus settle
    // windows); this guards that contract.
    let start = Instant::now();
    let _ = calibrated_shout("hello", 42.0);
    assert!(start.elapsed().as_millis() >= 10);
}

// --- is_palindrome --------------------------------------------------------

#[test]
fn palindrome_empty_is_true() {
    assert!(is_palindrome(""));
}

#[test]
fn palindrome_single_char_is_true() {
    assert!(is_palindrome("a"));
}

#[test]
fn palindrome_ignores_case_and_punctuation() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
}

#[test]
fn palindrome_detects_non_palindromes() {
    assert!(!is_palindrome("hello"));
}

#[test]
fn palindrome_emoji_only_filters_to_empty() {
    assert!(is_palindrome("\u{1F30D}\u{1F30D}"));
}

#[test]
fn palindrome_with_nul_bytes() {
    assert!(is_palindrome("a\0a"));
}

#[test]
fn palindrome_numbers_participate() {
    assert!(is_palindrome("1a2a1"));
    assert!(!is_palindrome("1a2a2"));
}

// --- word_frequency -------------------------------------------------------

#[test]
fn word_frequency_basic_counts() {
    let freqs = word_frequency("the cat and the dog and the bird the", 2);
    assert_eq!(freqs[0], ("the".to_string(), 4));
    assert_eq!(freqs.len(), 2);
}

#[test]
fn word_frequency_punctuation_only_yields_nothing() {
    assert!(word_frequency("!!! ??? ...", 10).is_empty());
}

#[test]
fn word_frequency_empty_text() {
    assert!(word_frequency("", 10).is_empty());
}

#[test]
fn word_frequency_is_case_insensitive() {
    let freqs = word_frequency("The the THE", 10);
    assert_eq!(freqs[0], ("the".to_string(), 3));
}

#[test]
fn word_frequency_numbers_are_words() {
    let freqs = word_frequency("123 abc 123", 10);
    assert_eq!(freqs[0], ("123".to_string(), 2));
}

#[test]
fn word_frequency_top_n_larger_than_vocabulary() {
    assert_eq!(word_frequency("a b a", 100).len(), 2);
}

#[test]
fn word_frequency_top_n_zero_returns_nothing() {
    assert!(word_frequency("a b c", 0).is_empty());
}

#[test]
fn word_frequency_ties_break_deterministically() {
    let freqs = word_frequency("cat dog", 10);
    assert_eq!(freqs[0].0, "dog");
    assert_eq!(freqs[1].0, "cat");
}

#[test]
fn word_frequency_combining_marks_split_words() {
    // 'e' + U+0301: the combining mark is not alphanumeric, so the word
    // breaks after 'e' by the per-char tokenization contract.
    let freqs = word_frequency("e\u{301}e", 10);
    assert_eq!(freqs[0], ("e".to_string(), 2));
}

// --- levenshtein ----------------------------------------------------------

#[test]
fn levenshtein_identical_is_zero() {
    assert_eq!(levenshtein("same", "same"), 0);
}

#[test]
fn levenshtein_empty_vs_word_is_length() {
    assert_eq!(levenshtein("", "abc"), 3);
    assert_eq!(levenshtein("abc", ""), 3);
}

#[test]
fn levenshtein_classic() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn levenshtein_unicode_chars_count_not_bytes() {
    assert_eq!(levenshtein("é", "e"), 1);
    assert_eq!(levenshtein("\u{1F30D}\u{1F30D}", "\u{1F30D}"), 1);
}

// --- compat layer ---------------------------------------------------------

#[test]
fn polyfill_probes_all_pass() {
    let report = apply_polyfills();
    assert!(report.string_uppercase_semantics);
    assert!(report.json_roundtrip_semantics);
    assert!(report.char_counting_semantics);
    assert_eq!(report.rechecks, 2);
}

#[test]
fn polyfill_probes_are_idempotent() {
    assert_eq!(apply_polyfills(), apply_polyfills());
}

#[test]
fn polyfill_probes_pass_concurrently() {
    let handles: Vec<_> = (0..8)
        .map(|_| thread::spawn(|| (0..100).map(|_| apply_polyfills()).collect::<Vec<_>>()))
        .collect();
    for handle in handles {
        for report in handle.join().expect("probe thread panicked") {
            assert!(report.string_uppercase_semantics);
            assert!(report.json_roundtrip_semantics);
            assert!(report.char_counting_semantics);
        }
    }
    assert_eq!(FAILED_PROBES.load(Ordering::SeqCst), 0);
}

#[test]
fn operation_counter_increases() {
    let before = OPERATION_COUNT.load(Ordering::SeqCst);
    let _ = reverse_string("abc");
    assert!(OPERATION_COUNT.load(Ordering::SeqCst) > before);
}

#[test]
fn operation_names_snapshot_does_not_alias_the_registry() {
    let _ = reverse_string("abc");
    let snapshot = operation_names();
    let before = snapshot.len();
    let _ = reverse_string("abcd");
    assert_eq!(snapshot.len(), before);
}

#[test]
fn deep_clone_severs_references() {
    let source = serde_json::json!({ "nested": { "list": [1, 2, 3] } });
    let mut clone = deep_clone(&source);
    assert_eq!(clone, source);
    clone["nested"]["list"][0] = serde_json::json!(99);
    assert_ne!(clone, source, "clone must not alias the source");
}

#[test]
fn stabilize_thread_sleeps_at_least_the_window() {
    let start = Instant::now();
    stabilize_thread(50);
    assert!(start.elapsed().as_millis() >= 45);
}

#[test]
fn warm_branch_predictor_counts_evens() {
    assert_eq!(warm_branch_predictor(10), 5);
    assert_eq!(warm_branch_predictor(0), 0);
    assert_eq!(warm_branch_predictor(1), 1);
    assert_eq!(warm_branch_predictor(20_000), 10_000);
}

#[test]
fn warm_branch_predictor_is_thread_safe() {
    let total: u64 = (0..4)
        .map(|_| thread::spawn(|| warm_branch_predictor(1000)))
        .map(|h| h.join().expect("warm-up thread panicked"))
        .sum();
    assert_eq!(total, 4 * 500);
}

#[test]
fn drama_entropy_matches_bit_population() {
    // The transmute preserves the exact register representation; the
    // population count must match `to_bits`.
    assert_eq!(
        drama_entropy(42.0),
        u64::from(42.0f64.to_bits().count_ones())
    );
    assert_eq!(drama_entropy(0.0), 0);
    assert_eq!(drama_entropy(1.0), u64::from(1.0f64.to_bits().count_ones()));
}

#[test]
fn fnv1a_digest_is_stable_and_different_per_token() {
    assert_eq!(fnv1a_hex("hunter2"), fnv1a_hex("hunter2"));
    assert_ne!(fnv1a_hex("hunter2"), fnv1a_hex("hunter3"));
    assert_eq!(fnv1a_hex("").len(), 16);
}

#[test]
fn fixed_token_truncates_ascii_to_capacity() {
    let long = "a".repeat(100);
    let token = FixedToken::<64>::new(&long);
    assert_eq!(token.as_str().len(), 64);
    assert_eq!(token.digest(), fnv1a_hex(&"a".repeat(64)));
}

#[test]
fn fixed_token_multibyte_cut_falls_back_to_the_empty_view() {
    // Documented behavior: a cut landing mid-character yields the empty
    // view rather than a panic; tokens are ASCII in practice.
    let token = FixedToken::<3>::new("éé"); // each 'é' is two bytes; the cut
                                            // at byte 3 lands inside the second character
    assert_eq!(token.as_str(), "");
    assert_eq!(token.digest(), fnv1a_hex(""));
}

#[test]
fn arc_report_clones_across_threads() {
    // The report must stay Send + Sync for cross-thread diagnostics.
    let report = Arc::new(apply_polyfills());
    let handle = thread::spawn(move || report.string_uppercase_semantics);
    assert!(handle.join().expect("report thread panicked"));
}

// --- cross-implementation parity -------------------------------------------

#[test]
fn parity_with_python_and_typescript_contracts() {
    // These exact values are asserted by the Python and TypeScript test
    // suites too; changing any of them is a breaking change.
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(shout("hello", 42.0), "HELLO!!!");
    assert!(is_palindrome("racecar"));
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}
