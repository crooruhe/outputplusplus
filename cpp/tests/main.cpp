// The suite entry point: run every registered case.
#include "framework.hpp"

int main() {
    return tdd::run_all() == 0 ? 0 : 1;
}
