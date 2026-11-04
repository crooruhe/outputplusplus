// The repo-wide parity harness: runs the core's parity check in
// process, then invokes the stack verifiers the way the docs do, and
// prints one line per stack.
//
// The aggregate view is informational: it reports what each verifier
// said without gating on it. Per-stack gates live in CI.

#include <cstdio>
#include <cstdlib>
#include <filesystem>
#include <string>
#include <sys/wait.h>

#include "outputplusplus/json.hpp"
#include "outputplusplus/parity.hpp"

namespace {

// Run one verifier command through the shell; returns its exit status,
// or -1 when the shell could not run it. The stream is flushed first so
// the child's output lands after the parent's lines.
int run_verifier(const std::string& command) {
    std::fflush(stdout);
    int status = std::system(command.c_str());
    if (status == -1) {
        return -1;
    }
    if (WIFEXITED(status)) {
        return WEXITSTATUS(status);
    }
    return -1;
}

const char* verdict(int status) {
    if (status == 0) {
        return "verified";
    }
    if (status > 0) {
        return "drift reported";
    }
    return "could not run";
}

}  // namespace

int main() {
    std::string canonical = opp::parity::find_canonical_path();
    if (canonical.empty()) {
        std::fprintf(stderr,
                     "opp-parity: parity/canonical.json not found; run from the "
                     "repository\n");
        return 1;
    }
    // The stack verifiers run from the repository root.
    std::filesystem::path repository_root =
        std::filesystem::path(canonical).parent_path().parent_path();
    std::error_code code;
    std::filesystem::current_path(repository_root, code);

    opp::JsonValue data = opp::parse_json(opp::read_file(canonical));
    opp::parity::Report report = opp::parity::check(data);
    std::printf("cpp   : %s\n", opp::parity::render(report).c_str());

    // The stack verifiers, invoked the same way the docs do.
    int python_status = run_verifier("opp verify-parity");
    std::printf("python: %s\n", verdict(python_status));
    int go_status = run_verifier("cd go && go run ./cli verify-parity");
    std::printf("go    : %s\n", verdict(go_status));

    std::printf("the aggregate view is informational; per-stack gates live in CI\n");
    return 0;
}
