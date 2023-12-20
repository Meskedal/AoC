#include <exception>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>
#include <regex>
#include <map>


/**
 * @brief Extracts the two digit number from a string as described in the problem of Day1 2023. 
 * Regex pattern matches a digit word (one, two, three, etc) or a digit (1, 2, 3, etc) and then optionally matches another digit word or digit.
 * @param str The string to extract the number from.
 * @return The integer two digit number.
 */
int extract_number(const std::string &str) {
    const static std::map<std::string, std::string> digit_word_to_digit_char = {
        {"one", "1"},
        {"two", "2"},
        {"three", "3"},
        {"four", "4"},
        {"five", "5"},
        {"six", "6"},
        {"seven", "7"},
        {"eight", "8"},
        {"nine", "9"},
        {"1", "1"},
        {"2", "2"},
        {"3", "3"},
        {"4", "4"},
        {"5", "5"},
        {"6", "6"},
        {"7", "7"},
        {"8", "8"},
        {"9", "9"}
    };
    const std::regex pattern("(one|two|three|four|five|six|seven|eight|nine|\\d)(?:.*(one|two|three|four|five|six|seven|eight|nine|\\d))?", std::regex::icase);
    std::smatch match;
    if (!std::regex_search(str, match, pattern)) {
        throw std::runtime_error("Could not match " + str);
    }
    std::string first_digit = match.str(1);
    std::string last_digit = match.str(2);
    if (last_digit == "") {
        last_digit = first_digit;
    }
    std::string combined_number = digit_word_to_digit_char.at(first_digit) + digit_word_to_digit_char.at(last_digit);
    return std::stoi(combined_number);
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>" << std::endl;
        return 1;
    } 

    std::ifstream input(argv[1]);
    if (!input.is_open()) {
        std::cerr << "Could not open file: " << argv[1] << std::endl;
        return 1;
    }

    int sum = 0;

    for (const auto &result : std::ranges::istream_view<std::string>(input) | std::views::transform([](auto &s) { return extract_number(s);})) {
        sum += result;
    }

    std::cout << "Sum: " << sum << std::endl;
    return 0;
}