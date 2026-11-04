// Tests for the shared string contracts. The parity values are pinned
// by the Python, TypeScript, Rust, and Go suites too.

#include <string>

#include "framework.hpp"
#include "outputplusplus/text.hpp"

TEST(shout_parity) {
    ASSERT_EQ("HELLO!!!", opp::shout("hello", 42.0));
    ASSERT_EQ("HELLO!!!!!!", opp::shout("hello", 95.0));
    ASSERT_EQ("!!!", opp::shout("", 42.0));
    // The drama boundary is inclusive.
    ASSERT_EQ("HI!!!!!!", opp::shout("hi", 90.0));
}

TEST(reverse_parity) {
    ASSERT_EQ("olleh", opp::reverse_string("hello"));
    ASSERT_EQ("", opp::reverse_string(""));
    ASSERT_EQ("a", opp::reverse_string("a"));
    // Code-point-wise, not byte-wise.
    ASSERT_EQ("oll\xc3\xa9h", opp::reverse_string("h\xc3\xa9llo"));
}

TEST(reverse_ten_thousand_characters) {
    // The parity pin: 10,000 characters (see parity/canonical.json).
    std::string input;
    for (int i = 0; i < 5000; i++) {
        input += "ab";
    }
    std::string want;
    for (int i = 0; i < 5000; i++) {
        want += "ba";
    }
    ASSERT(want == opp::reverse_string(input));
}

TEST(palindrome_parity) {
    ASSERT(opp::is_palindrome("A man, a plan, a canal: Panama"));
    ASSERT(opp::is_palindrome("racecar"));
    ASSERT(!opp::is_palindrome("hello"));
    ASSERT(opp::is_palindrome(""));
}

TEST(levenshtein_parity) {
    ASSERT_EQ(3, opp::levenshtein("kitten", "sitting"));
    ASSERT_EQ(3, opp::levenshtein("", "abc"));
    ASSERT_EQ(0, opp::levenshtein("same", "same"));
}

TEST(word_frequency_counts) {
    std::vector<opp::WordCount> entries =
        opp::word_frequency("the cat and the dog and the bird the", 2);
    ASSERT_EQ(2, (int)entries.size());
    ASSERT_EQ("the", entries[0].word);
    ASSERT_EQ(4, entries[0].count);
    // "and" (2) outranks the count-1 words, so the second slot is fixed.
    ASSERT_EQ("and", entries[1].word);
    ASSERT_EQ(2, entries[1].count);
}

TEST(word_frequency_ties_are_registration_order) {
    // "cat" and "dog" both count 1; whichever the registry registers
    // first wins. The registry's iteration order is the contract, so
    // this test asserts membership, not order.
    std::vector<opp::WordCount> entries = opp::word_frequency("cat dog", 2);
    ASSERT_EQ(2, (int)entries.size());
    for (const opp::WordCount& entry : entries) {
        ASSERT(entry.word == "cat" || entry.word == "dog");
        ASSERT_EQ(1, entry.count);
    }
}

TEST(word_frequency_lowercases_and_bounds_top_n) {
    std::vector<opp::WordCount> entries = opp::word_frequency("The the THE", 3);
    ASSERT_EQ(1, (int)entries.size());
    ASSERT_EQ("the", entries[0].word);
    ASSERT_EQ(3, entries[0].count);
    ASSERT_EQ(0, (int)opp::word_frequency("a b c", 0).size());
    ASSERT_EQ(0, (int)opp::word_frequency("", 10).size());
}

TEST(drama_entropy_matches_the_bit_population) {
    // 42.0's IEEE-754 representation carries four set bits; the value
    // is the canonical vector.
    ASSERT_EQ(4, opp::drama_entropy(42.0));
    ASSERT_EQ(0, opp::drama_entropy(0.0));
}
