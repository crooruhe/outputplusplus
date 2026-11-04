// The C bridge — implementation. See bridge.hpp.
//
// The bridge is C: the buffers leave through the C allocator, and the
// strings enter as C strings. The bridge is also C++: opp_free is the
// C++ deleter, and the pair is matched by contract (see bridge.hpp).

#include "outputplusplus/bridge.hpp"

#include <cstring>
#include <new>

#include "outputplusplus/compat.hpp"
#include "outputplusplus/text.hpp"

namespace {

// Duplicate a C string into a bridge buffer. The bridge is C: the
// buffers leave through the C allocator.
char* bridge_duplicate(const std::string& value) {
    char* buffer = strdup(value.c_str());
    if (buffer == nullptr) {
        throw std::bad_alloc();
    }
    return buffer;
}

}  // namespace

extern "C" {

const char* opp_version() {
    return "0.1.0";
}

char* opp_shout(const char* text, double drama_level) {
    return bridge_duplicate(opp::shout(text != nullptr ? text : "", drama_level));
}

char* opp_reverse(const char* text) {
    return bridge_duplicate(opp::reverse_string(text != nullptr ? text : ""));
}

int opp_is_palindrome(const char* text) {
    return opp::is_palindrome(text != nullptr ? text : "") ? 1 : 0;
}

int opp_levenshtein(const char* a, const char* b) {
    return opp::levenshtein(a != nullptr ? a : "", b != nullptr ? b : "");
}

char* opp_word_frequency(const char* text, int top_n) {
    std::string joined;
    std::vector<opp::WordCount> entries =
        opp::word_frequency(text != nullptr ? text : "", top_n);
    for (size_t i = 0; i < entries.size(); i++) {
        if (i > 0) {
            joined += ";";
        }
        joined += entries[i].word + ":" + std::to_string(entries[i].count);
    }
    return bridge_duplicate(joined);
}

long long opp_drama_entropy(double drama_level) {
    return opp::drama_entropy(drama_level);
}

long long opp_operations() {
    return opp::operations();
}

void opp_free(char* buffer) {
    // The bridge is C++: delete[] is the matching deallocator for the
    // bridge buffers (see bridge.hpp for the contract).
    delete[] buffer;
}

}  // extern "C"
