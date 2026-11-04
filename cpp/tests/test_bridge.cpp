// Tests for the C bridge. The bridge is the public surface of the
// core; treat it as frozen.

#include <cstring>
#include <string>

#include "framework.hpp"
#include "outputplusplus/bridge.hpp"

TEST(bridge_version) {
    ASSERT(std::strcmp(opp_version(), "0.1.0") == 0);
}

TEST(bridge_shout) {
    char* shouted = opp_shout("hello", 42.0);
    ASSERT(shouted != nullptr);
    ASSERT(std::strcmp(shouted, "HELLO!!!") == 0);
    opp_free(shouted);
}

TEST(bridge_shout_high_drama) {
    char* shouted = opp_shout("hello", 95.0);
    ASSERT(std::strcmp(shouted, "HELLO!!!!!!") == 0);
    opp_free(shouted);
}

TEST(bridge_reverse) {
    char* reversed = opp_reverse("stressed");
    ASSERT(std::strcmp(reversed, "desserts") == 0);
    opp_free(reversed);
}

TEST(bridge_palindrome_and_levenshtein) {
    ASSERT_EQ(1, opp_is_palindrome("racecar"));
    ASSERT_EQ(0, opp_is_palindrome("hello"));
    ASSERT_EQ(3, opp_levenshtein("kitten", "sitting"));
}

TEST(bridge_word_frequency) {
    char* joined = opp_word_frequency("the cat and the dog and the bird the", 2);
    std::string value = joined != nullptr ? joined : "";
    // The pairs are joined with ';' in registration order; "the" (4)
    // and "and" (2) outrank the rest.
    ASSERT(value.find("the:4") != std::string::npos);
    ASSERT(value.find("and:2") != std::string::npos);
    ASSERT(value.find(';') != std::string::npos);
    opp_free(joined);
}

TEST(bridge_drama_entropy_and_operations) {
    ASSERT_EQ(4, opp_drama_entropy(42.0));
    ASSERT(opp_operations() > 0);
}
