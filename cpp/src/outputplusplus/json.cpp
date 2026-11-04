// A minimal JSON value and parser — implementation. See json.hpp.

#include "outputplusplus/json.hpp"

#include <cctype>
#include <cstdio>
#include <fstream>
#include <sstream>
#include <stdexcept>

namespace opp {

std::string read_file(const std::string& path) {
    std::ifstream stream(path, std::ios::binary);
    if (!stream) {
        throw std::runtime_error("cannot read " + path);
    }
    std::ostringstream buffer;
    buffer << stream.rdbuf();
    return buffer.str();
}

namespace {

// The parser: a recursive descent over the source. The pointer walks
// the document; every branch advances it before recursing.
struct Parser {
    const std::string& source;
    size_t position = 0;

    explicit Parser(const std::string& source_) : source(source_) {}

    void skip_whitespace() {
        while (position < source.size() &&
               (source[position] == ' ' || source[position] == '\n' ||
                source[position] == '\t' || source[position] == '\r')) {
            position = position + 1;
        }
    }

    char peek() {
        if (position >= source.size()) {
            throw std::runtime_error("unexpected end of document");
        }
        return source[position];
    }

    void expect(char expected) {
        if (peek() != expected) {
            throw std::runtime_error(std::string("expected '") + expected +
                                     "' at position " + std::to_string(position));
        }
        position = position + 1;
    }

    bool match(char expected) {
        if (position < source.size() && source[position] == expected) {
            position = position + 1;
            return true;
        }
        return false;
    }

    JsonValue parse_value() {
        skip_whitespace();
        char next = peek();
        if (next == '{') {
            return parse_object();
        }
        if (next == '[') {
            return parse_array();
        }
        if (next == '"') {
            JsonValue value;
            value.kind = JsonValue::Kind::Text;
            value.text = parse_string();
            return value;
        }
        if (next == 't' || next == 'f') {
            return parse_boolean();
        }
        if (next == 'n') {
            expect('n');
            expect('u');
            expect('l');
            expect('l');
            return JsonValue();
        }
        return parse_number();
    }

    JsonValue parse_object() {
        expect('{');
        JsonValue value;
        value.kind = JsonValue::Kind::Object;
        skip_whitespace();
        if (match('}')) {
            return value;
        }
        while (true) {
            skip_whitespace();
            std::string key = parse_string();
            skip_whitespace();
            expect(':');
            value.object.emplace_back(key, parse_value());
            skip_whitespace();
            if (match(',')) {
                continue;
            }
            expect('}');
            return value;
        }
    }

    JsonValue parse_array() {
        expect('[');
        JsonValue value;
        value.kind = JsonValue::Kind::Array;
        skip_whitespace();
        if (match(']')) {
            return value;
        }
        while (true) {
            value.array.push_back(parse_value());
            skip_whitespace();
            if (match(',')) {
                continue;
            }
            expect(']');
            return value;
        }
    }

    std::string parse_string() {
        expect('"');
        std::string out;
        while (true) {
            char character = peek();
            position = position + 1;
            if (character == '"') {
                return out;
            }
            if (character != '\\') {
                out += character;
                continue;
            }
            // Escape sequences: the canonical file uses the standard set.
            char escaped = peek();
            position = position + 1;
            switch (escaped) {
                case '"': out += '"'; break;
                case '\\': out += '\\'; break;
                case '/': out += '/'; break;
                case 'b': out += '\b'; break;
                case 'f': out += '\f'; break;
                case 'n': out += '\n'; break;
                case 'r': out += '\r'; break;
                case 't': out += '\t'; break;
                default: out += escaped; break;
            }
        }
    }

    JsonValue parse_boolean() {
        JsonValue value;
        value.kind = JsonValue::Kind::Boolean;
        if (match('t')) {
            expect('r');
            expect('u');
            expect('e');
            value.boolean = true;
            return value;
        }
        expect('f');
        expect('a');
        expect('l');
        expect('s');
        expect('e');
        value.boolean = false;
        return value;
    }

    JsonValue parse_number() {
        size_t start = position;
        while (position < source.size() &&
               (isdigit((unsigned char)source[position]) ||
                source[position] == '-' || source[position] == '+' ||
                source[position] == '.' || source[position] == 'e' ||
                source[position] == 'E')) {
            position = position + 1;
        }
        if (start == position) {
            throw std::runtime_error("unexpected character at position " +
                                     std::to_string(position));
        }
        JsonValue value;
        value.kind = JsonValue::Kind::Number;
        value.number = std::stod(source.substr(start, position - start));
        return value;
    }
};

}  // namespace

JsonValue parse_json(const std::string& source) {
    Parser parser(source);
    JsonValue value = parser.parse_value();
    parser.skip_whitespace();
    if (parser.position != source.size()) {
        throw std::runtime_error("trailing content at position " +
                                 std::to_string(parser.position));
    }
    return value;
}

}  // namespace opp
