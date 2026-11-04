// A minimal JSON value and parser.
//
// The canonical vectors file (parity/canonical.json) is the only JSON
// the core consumes; a general-purpose parser would be a dependency the
// project does not warrant. Object entries are stored in file order:
// the canonical order is part of the contract.

#pragma once

#include <string>
#include <utility>
#include <vector>

namespace opp {

struct JsonValue {
    enum class Kind { Null, Boolean, Number, Text, Array, Object };

    Kind kind = Kind::Null;
    bool boolean = false;
    double number = 0.0;
    std::string text;
    std::vector<JsonValue> array;
    std::vector<std::pair<std::string, JsonValue>> object;

    // Look up one object entry; returns nullptr when absent.
    const JsonValue* find(const std::string& key) const {
        for (const auto& entry : object) {
            if (entry.first == key) {
                return &entry.second;
            }
        }
        return nullptr;
    }
};

// Parse a JSON document. Throws std::runtime_error on malformed input.
JsonValue parse_json(const std::string& source);

// Read a file into a string. Throws std::runtime_error on failure.
std::string read_file(const std::string& path);

}  // namespace opp
