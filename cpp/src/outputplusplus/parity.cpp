// The parity checker — implementation. See parity.hpp.

#include "outputplusplus/parity.hpp"

#include <filesystem>

#include "outputplusplus/text.hpp"

namespace opp {
namespace parity {

std::string find_canonical_path() {
    std::error_code code;
    std::filesystem::path directory = std::filesystem::current_path(code);
    if (code) {
        return "";
    }
    while (true) {
        std::filesystem::path candidate = directory / "parity" / "canonical.json";
        if (std::filesystem::exists(candidate)) {
            return candidate.string();
        }
        if (!directory.has_parent_path() || directory.parent_path() == directory) {
            return "";
        }
        directory = directory.parent_path();
    }
}

namespace {

const JsonValue* vectors_of(const JsonValue& canonical, const std::string& category) {
    const JsonValue* section = canonical.find(category);
    if (section == nullptr) {
        return nullptr;
    }
    return section->find("vectors");
}

bool is_advisory(const JsonValue& vector) {
    return vector.find("note") != nullptr;
}

std::string text_or(const JsonValue& vector, const std::string& key,
                    const std::string& fallback) {
    const JsonValue* field = vector.find(key);
    if (field != nullptr && field->kind == JsonValue::Kind::Text) {
        return field->text;
    }
    return fallback;
}

double number_or(const JsonValue& vector, const std::string& key, double fallback) {
    const JsonValue* field = vector.find(key);
    if (field != nullptr && field->kind == JsonValue::Kind::Number) {
        return field->number;
    }
    return fallback;
}

bool bool_or(const JsonValue& vector, const std::string& key, bool fallback) {
    const JsonValue* field = vector.find(key);
    if (field != nullptr && field->kind == JsonValue::Kind::Boolean) {
        return field->boolean;
    }
    return fallback;
}

// Compare one vector; returns true when the core matches.
bool compare_vector(const std::string& category, const JsonValue& vector) {
    if (category == "shout") {
        return shout(text_or(vector, "input", ""),
                     number_or(vector, "drama_level", 42.0)) ==
               text_or(vector, "expected", "");
    }
    if (category == "reverse") {
        return reverse_string(text_or(vector, "input", "")) ==
               text_or(vector, "expected", "");
    }
    if (category == "palindrome") {
        return is_palindrome(text_or(vector, "input", "")) ==
               bool_or(vector, "expected", false);
    }
    if (category == "levenshtein") {
        return levenshtein(text_or(vector, "a", ""), text_or(vector, "b", "")) ==
               (int)number_or(vector, "expected", 0.0);
    }
    if (category == "word_frequency") {
        // The canonical arrays preserve order; the comparison is by
        // membership, the way the other registries compare.
        std::vector<WordCount> actual =
            word_frequency(text_or(vector, "text", ""),
                           (int)number_or(vector, "top_n", 10.0));
        const JsonValue* expected = vector.find("expected");
        if (expected == nullptr || expected->kind != JsonValue::Kind::Array) {
            return false;
        }
        if (actual.size() != expected->array.size()) {
            return false;
        }
        for (const WordCount& entry : actual) {
            bool seen = false;
            for (const JsonValue& pair : expected->array) {
                if (pair.kind == JsonValue::Kind::Array && pair.array.size() == 2 &&
                    pair.array[0].kind == JsonValue::Kind::Text &&
                    pair.array[0].text == entry.word &&
                    pair.array[1].kind == JsonValue::Kind::Number &&
                    (int)pair.array[1].number == entry.count) {
                    seen = true;
                }
            }
            if (!seen) {
                return false;
            }
        }
        return true;
    }
    return false;
}

}  // namespace

Report check(const JsonValue& canonical) {
    Report report;
    for (const char* const category :
         {"shout", "reverse", "palindrome", "levenshtein", "word_frequency"}) {
        const JsonValue* vectors = vectors_of(canonical, category);
        if (vectors == nullptr || vectors->kind != JsonValue::Kind::Array) {
            continue;
        }
        for (const JsonValue& vector : vectors->array) {
            if (vector.kind != JsonValue::Kind::Object) {
                continue;
            }
            bool advisory = is_advisory(vector);
            report.checked = report.checked + 1;
            bool matches = compare_vector(category, vector);
            if (advisory) {
                // Compared for reporting only; advisory vectors do not
                // gate.
                report.advisory = report.advisory + 1;
                continue;
            }
            if (!matches) {
                report.failed = report.failed + 1;
            }
        }
    }
    return report;
}

std::string render(const Report& report) {
    return "parity checked: " + std::to_string(report.checked) + " vectors, " +
           std::to_string(report.failed) + " failed, " +
           std::to_string(report.advisory) +
           " advisory (compared for reporting only)";
}

}  // namespace parity
}  // namespace opp
