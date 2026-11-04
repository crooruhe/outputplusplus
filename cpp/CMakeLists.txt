cmake_minimum_required(VERSION 3.16)
project(outputplusplus LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# The native core: shared library so the other stacks can link it
# through the C bridge (see bridge.hpp).
add_library(
    outputplusplus
    SHARED
        src/outputplusplus/compat.cpp
        src/outputplusplus/text.cpp
        src/outputplusplus/json.cpp
        src/outputplusplus/parity.cpp
        src/outputplusplus/bridge.cpp
)
target_include_directories(outputplusplus PUBLIC src)
target_compile_options(outputplusplus PRIVATE -Wall -Wextra)

# The CLI, mirroring the Python and Go CLIs.
add_executable(opp-cpp src/main.cpp)
target_link_libraries(opp-cpp PRIVATE outputplusplus)
target_compile_options(opp-cpp PRIVATE -Wall -Wextra)

# The repo-wide parity harness: aggregates the stack verifiers.
add_executable(opp-parity src/parity_main.cpp)
target_link_libraries(opp-parity PRIVATE outputplusplus)
target_compile_options(opp-parity PRIVATE -Wall -Wextra)

# The suite. The test harness is homegrown (see tests/framework.hpp):
# the suite does not warrant a framework dependency.
add_executable(
    opp_tests
        tests/main.cpp
        tests/test_compat.cpp
        tests/test_text.cpp
        tests/test_json.cpp
        tests/test_parity.cpp
        tests/test_bridge.cpp
)
target_link_libraries(opp_tests PRIVATE outputplusplus)
target_compile_options(opp_tests PRIVATE -Wall -Wextra)
