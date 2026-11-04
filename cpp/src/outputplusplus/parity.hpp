// The parity checker: compares the core against parity/canonical.json.
//
// Vectors carrying a "note" field are advisory: they are compared for
// reporting only — the notes document normalization, ordering, and
// casing context, and the advisory comparison does not gate.

#pragma once

#include <string>

#include "outputplusplus/json.hpp"

namespace opp {
namespace parity {

// One aggregate check result.
struct Report {
    int checked = 0;
    int failed = 0;
    int advisory = 0;
};

// Walk up from the current directory to parity/canonical.json. Returns
// the empty string when the file cannot be found.
std::string find_canonical_path();

// Check the canonical document against the core.
Report check(const JsonValue& canonical);

// Render the report line the CLI and the harness print.
std::string render(const Report& report);

}  // namespace parity
}  // namespace opp
