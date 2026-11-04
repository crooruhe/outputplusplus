// The C bridge: the native core's stable ABI for the other stacks.
//
// The Python package consumes this bridge through ctypes (see
// python/src/outputplusplus/native.py); the Rust and Go projects can
// link it the same way. The bridge is the public surface of the core —
// treat it as frozen.

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// The core version, as a static string. The caller does not own the
// buffer.
const char* opp_version();

// SHOUT the text back (see text.hpp). The caller releases the returned
// buffer with opp_free.
char* opp_shout(const char* text, double drama_level);

// Reverse the text, code point by code point. The caller releases the
// returned buffer with opp_free.
char* opp_reverse(const char* text);

// 1 when the text is a palindrome, 0 otherwise.
int opp_is_palindrome(const char* text);

// Edit distance between two texts.
int opp_levenshtein(const char* a, const char* b);

// Word frequencies as "word:count" pairs joined with ';' (top_n
// entries). The caller releases the returned buffer with opp_free.
char* opp_word_frequency(const char* text, int top_n);

// Population count of the level's IEEE-754 bit pattern.
long long opp_drama_entropy(double drama_level);

// The number of operations the core has recorded.
long long opp_operations();

// Release a buffer returned by the bridge.
void opp_free(char* buffer);

#ifdef __cplusplus
}
#endif
