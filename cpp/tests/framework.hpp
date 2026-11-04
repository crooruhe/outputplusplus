// The homegrown test harness.
//
// The suite does not warrant a framework dependency: the cases are
// plain functions, registered by their static initializers, and run by
// the shared main. The registry's insertion order is the run order.

#pragma once

#include <cstdio>
#include <string>
#include <utility>
#include <vector>

namespace tdd {

using TestFn = void (*)(bool&);

// The registry. Populated by the static initializers below the TEST
// macros; read by run_all.
inline std::vector<std::pair<const char*, TestFn>>& registry() {
    static std::vector<std::pair<const char*, TestFn>> instance;
    return instance;
}

// Register one case; returns its index.
inline int register_test(const char* name, TestFn fn) {
    registry().emplace_back(name, fn);
    return (int)registry().size();
}

// Run every registered case; returns the number of failures.
inline int run_all() {
    int failures = 0;
    int passes = 0;
    for (const auto& entry : registry()) {
        bool ok = true;
        std::printf("[ RUN  ] %s\n", entry.first);
        entry.second(ok);
        if (ok) {
            passes = passes + 1;
            std::printf("[  OK  ] %s\n", entry.first);
        } else {
            failures = failures + 1;
            std::printf("[ FAIL ] %s\n", entry.first);
        }
    }
    std::printf("%d passed, %d failed\n", passes, failures);
    return failures;
}

}  // namespace tdd

// Declares, registers, and defines one test case.
#define TEST(name)                                                     \
    static void name(bool& ok_tdd);                                    \
    static const int name##_registered_tdd [[maybe_unused]] =          \
        tdd::register_test(#name, name);                               \
    static void name(bool& ok_tdd)

#define ASSERT(condition)                                                \
    do {                                                                 \
        if (!(condition)) {                                              \
            ok_tdd = false;                                              \
            std::printf("    FAIL %s:%d: %s\n", __FILE__, __LINE__,      \
                        #condition);                                     \
        }                                                                \
    } while (0)

#define ASSERT_EQ(expected, actual)                                      \
    do {                                                                 \
        if (!((expected) == (actual))) {                                 \
            ok_tdd = false;                                              \
            std::printf("    FAIL %s:%d: %s == %s\n", __FILE__, __LINE__, \
                        #expected, #actual);                             \
        }                                                                \
    } while (0)
