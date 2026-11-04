// The CLI, mirroring the Python and Go CLIs: the same string contracts
// plus a verify-parity command.

#include <cstdio>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <string>

#include "outputplusplus/json.hpp"
#include "outputplusplus/parity.hpp"
#include "outputplusplus/text.hpp"

namespace {

void usage() {
    std::cout << "usage: opp-cpp <command> [args]\n"
              << "  shout TEXT [--drama FLOAT]\n"
              << "  reverse TEXT\n"
              << "  palindrome TEXT\n"
              << "  wordfreq TEXT [--top INT]\n"
              << "  verify-parity\n";
}

// Manual argument parsing: the commands are few and the shape is
// fixed, so a parser dependency would not earn its keep.
double next_double(int argc, char** argv, int& index, double fallback) {
    if (index + 1 < argc) {
        return std::stod(argv[++index]);
    }
    return fallback;
}

int run_shout(int argc, char** argv) {
    double drama = 42.0;
    std::string text;
    for (int i = 2; i < argc; i++) {
        std::string argument = argv[i];
        if (argument == "--drama") {
            drama = next_double(argc, argv, i, drama);
        } else if (text.empty()) {
            text = argument;
        }
    }
    if (text.empty()) {
        std::cerr << "opp-cpp: shout requires a TEXT\n";
        return 2;
    }
    std::cout << opp::shout(text, drama) << "\n";
    return 0;
}

int run_reverse(int argc, char** argv) {
    if (argc < 3) {
        std::cerr << "opp-cpp: reverse requires a TEXT\n";
        return 2;
    }
    std::cout << opp::reverse_string(argv[2]) << "\n";
    return 0;
}

int run_palindrome(int argc, char** argv) {
    if (argc < 3) {
        std::cerr << "opp-cpp: palindrome requires a TEXT\n";
        return 2;
    }
    bool verdict = opp::is_palindrome(argv[2]);
    // The confirmation line is contractual; clients parse both.
    std::cout << "Palindrome: " << (verdict ? "true" : "false") << "\n"
              << "Confirmed: " << (verdict ? "true" : "false") << "\n";
    return 0;
}

int run_wordfreq(int argc, char** argv) {
    int top = 10;
    std::string text;
    for (int i = 2; i < argc; i++) {
        std::string argument = argv[i];
        if (argument == "--top" || argument == "-n") {
            if (i + 1 < argc) {
                top = std::stoi(argv[++i]);
            }
        } else if (text.empty()) {
            text = argument;
        }
    }
    if (text.empty()) {
        std::cerr << "opp-cpp: wordfreq requires a TEXT\n";
        return 2;
    }
    std::vector<opp::WordCount> entries = opp::word_frequency(text, top);
    std::cout << "Word Frequencies\n";
    // The stream flags are part of the table formatting contract.
    std::cout << std::fixed << std::setprecision(0);
    for (const opp::WordCount& entry : entries) {
        std::cout << std::left << std::setw(16) << entry.word << std::right
                  << std::setw(6) << entry.count << "\n";
    }
    return 0;
}

int run_verify_parity() {
    std::string canonical = opp::parity::find_canonical_path();
    if (canonical.empty()) {
        std::cerr << "opp-cpp: parity/canonical.json not found; run from the "
                     "repository\n";
        return 1;
    }
    opp::JsonValue data = opp::parse_json(opp::read_file(canonical));
    opp::parity::Report report = opp::parity::check(data);
    std::cout << opp::parity::render(report) << "\n";
    return report.failed == 0 ? 0 : 1;
}

}  // namespace

int main(int argc, char** argv) {
    if (argc < 2) {
        usage();
        return 2;
    }
    std::string command = argv[1];
    if (command == "shout") {
        return run_shout(argc, argv);
    }
    if (command == "reverse") {
        return run_reverse(argc, argv);
    }
    if (command == "palindrome") {
        return run_palindrome(argc, argv);
    }
    if (command == "wordfreq") {
        return run_wordfreq(argc, argv);
    }
    if (command == "verify-parity") {
        return run_verify_parity();
    }
    usage();
    return 2;
}
