// Tests for the parity checker.

#include "framework.hpp"
#include "outputplusplus/json.hpp"
#include "outputplusplus/parity.hpp"

TEST(check_the_canonical_file) {
    std::string path = opp::parity::find_canonical_path();
    if (path.empty()) {
        // The harness runs from the repository root via the Makefile.
        return;
    }
    opp::JsonValue canonical = opp::parse_json(opp::read_file(path));
    opp::parity::Report report = opp::parity::check(canonical);
    // Every vector is checked; the advisory ones (the ones with notes)
    // are compared for reporting only and do not gate.
    ASSERT_EQ(14, report.checked);
    ASSERT_EQ(0, report.failed);
    ASSERT_EQ(4, report.advisory);
}

TEST(drifted_vectors_fail) {
    // A drifted vector without a note gates.
    opp::JsonValue canonical = opp::parse_json(
        "{\"reverse\": {\"vectors\": [{\"input\": \"stressed\", "
        "\"expected\": \"WRONG\"}]}}");
    opp::parity::Report report = opp::parity::check(canonical);
    ASSERT_EQ(1, report.checked);
    ASSERT_EQ(1, report.failed);
    ASSERT_EQ(0, report.advisory);
}

TEST(advisory_vectors_do_not_gate) {
    // The same drift, but advisory: documented, not asserted.
    opp::JsonValue canonical = opp::parse_json(
        "{\"reverse\": {\"vectors\": [{\"input\": \"stressed\", "
        "\"expected\": \"WRONG\", \"note\": \"advisory\"}]}}");
    opp::parity::Report report = opp::parity::check(canonical);
    ASSERT_EQ(1, report.checked);
    ASSERT_EQ(0, report.failed);
    ASSERT_EQ(1, report.advisory);
}

TEST(render_reports_the_counts) {
    opp::parity::Report report;
    report.checked = 13;
    report.failed = 0;
    report.advisory = 3;
    std::string line = opp::parity::render(report);
    ASSERT(line.find("13 vectors") != std::string::npos);
    ASSERT(line.find("3 advisory") != std::string::npos);
}
