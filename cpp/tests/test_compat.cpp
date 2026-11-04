// Tests for the compatibility layer.

#include <chrono>

#include "framework.hpp"
#include "outputplusplus/compat.hpp"

TEST(polyfill_probes_all_pass) {
    opp::apply_polyfills();
    for (const auto& entry : opp::polyfills_applied) {
        if (entry.first == "fallback_engaged") {
            continue;
        }
        ASSERT(entry.second);
    }
    ASSERT(opp::polyfills_applied["cctype_toupper"]);
    ASSERT(opp::polyfills_applied["string_bytes"]);
}

TEST(count_operation_increments) {
    long long before = opp::operations();
    opp::count_operation("probe");
    ASSERT(opp::operations() > before);
}

TEST(warm_branch_predictor_counts_evens) {
    ASSERT_EQ(5, opp::warm_branch_predictor(10));
    ASSERT_EQ(0, opp::warm_branch_predictor(0));
    ASSERT_EQ(10000, opp::warm_branch_predictor(20000));
}

TEST(deep_clone_severs_references) {
    std::vector<std::string> source = {"a", "b", "c"};
    std::vector<std::string> cloned = opp::deep_clone(source);
    ASSERT_EQ(3, (int)cloned.size());
    cloned[0] = "mutated";
    ASSERT(source[0] == "a");
}

TEST(fnv1a_is_stable_and_discriminating) {
    ASSERT(opp::fnv1a_hex("hunter2") == opp::fnv1a_hex("hunter2"));
    ASSERT(opp::fnv1a_hex("hunter2") != opp::fnv1a_hex("hunter3"));
    ASSERT_EQ(16, (int)opp::fnv1a_hex("").size());
}

TEST(stabilize_thread_sleeps_at_least_the_window) {
    auto start = std::chrono::steady_clock::now();
    opp::stabilize_thread(50);
    auto elapsed = std::chrono::steady_clock::now() - start;
    ASSERT(std::chrono::duration_cast<std::chrono::milliseconds>(elapsed).count() >=
           45);
}
