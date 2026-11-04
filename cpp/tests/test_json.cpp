// Tests for the JSON value and parser.

#include "framework.hpp"
#include "outputplusplus/json.hpp"
#include "outputplusplus/parity.hpp"

#include <stdexcept>

TEST(parse_scalars_and_containers) {
    opp::JsonValue value = opp::parse_json(
        "{\"ok\": true, \"n\": -1.5, \"s\": \"a\\\"b\", \"list\": [1, 2, 3]}");
    ASSERT_EQ((int)opp::JsonValue::Kind::Object, (int)value.kind);
    ASSERT(value.find("ok") != nullptr && value.find("ok")->boolean);
    ASSERT(value.find("n") != nullptr && value.find("n")->number == -1.5);
    ASSERT(value.find("s") != nullptr && value.find("s")->text == "a\"b");
    ASSERT(value.find("list") != nullptr && value.find("list")->array.size() == 3);
    ASSERT(value.find("missing") == nullptr);
}

TEST(object_entries_preserve_file_order) {
    opp::JsonValue value = opp::parse_json("{\"z\": 1, \"a\": 2}");
    ASSERT_EQ("z", value.object[0].first);
    ASSERT_EQ("a", value.object[1].first);
}

TEST(malformed_documents_raise) {
    bool raised = false;
    try {
        opp::parse_json("{\"unterminated\": ");
    } catch (const std::runtime_error&) {
        raised = true;
    }
    ASSERT(raised);
}

TEST(parse_the_canonical_file) {
    // The canonical vectors file is the only JSON the core consumes;
    // the parser reads it in full.
    std::string path = opp::parity::find_canonical_path();
    if (path.empty()) {
        // Run from the repository; the parity tests cover the check.
        return;
    }
    opp::JsonValue canonical = opp::parse_json(opp::read_file(path));
    ASSERT(canonical.find("shout") != nullptr);
    ASSERT(canonical.find("word_frequency") != nullptr);
    ASSERT(canonical.find("science") != nullptr);
}
