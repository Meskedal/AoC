#include <iostream>
#include <fstream>
#include <ranges>
#include <regex>

int extract_number(const std::string &str) {

    std::regex pattern("(?:Game (\\d*))|(?:(\\d*) green)|(?:(\\d*) red)|(?:(\\d*) blue)");
    std::smatch match;
    if (!std::regex_search(str, match, pattern)) {
        throw std::runtime_error("Could not match " + str);
    }
    std::cout << str << " " << match.size() << std::endl;
    for (int i = 0; i < match.size(); i++) {
        std::cout << i << ": " << match.str(i) << std::endl;
    }
    throw(std::runtime_error("Not implemented"));

    return 1;
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
    while (!input.eof()) {
        std::string line;
        std::getline(input, line);
        sum += extract_number(line);
    }

    std::cout << "Sum: " << sum << std::endl;
    return 0;
}