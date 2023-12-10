#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>
#include <regex>
#include <map>

std::map<std::string, std::string> digit_word_to_digit_char = {
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
    const std::regex pattern("(one|two|three|four|five|six|seven|eight|nine|\\d)(?:.*(one|two|three|four|five|six|seven|eight|nine|\\d))?.*");
    // const std::regex first_pattern("(one|two|three|four|five|six|seven|eight|nine|\\d)", std::regex::icase);
    // const std::regex last_pattern(".*(one|two|three|four|five|six|seven|eight|nine|\\d).*?$", std::regex::icase);

    auto regex_match = [pattern](auto &s) { 
        std::smatch match;
        std::smatch last_match;
        std::regex_search(s, match, pattern);
        if (match.size() < 2) {
            std::cerr << "No matched group " << s << std::endl;
            return 1;
        }

        std::string first_digit = match.str(1);
        std::string last_digit = match.size() == 2 ? match.str(2) : match.str(3);

        // if (!std::regex_search(s, last_match, last_pattern)) {
        //     std::cerr << "Could not find last match in: " << s << std::endl;
        //     return 1;
        // }

        // std::string first_digit = match.str(1);
        // std::string last_digit = last_match.str(1);
        std::string combined_number = digit_word_to_digit_char[first_digit] + digit_word_to_digit_char[last_digit];
        return std::stoi(combined_number);
    };

    int sum = 0;

    for (const auto &result : std::ranges::istream_view<std::string>(input) | std::views::transform([regex_match](auto &s) { return regex_match(s);})) {
        sum += result;
    }

    std::cout << "Sum: " << sum << std::endl;
}