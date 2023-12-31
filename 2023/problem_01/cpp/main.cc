/// main.cc
///
/// Advent of Code 2023, December 1: Trebuchet, Part 1&2
///
/// Copyright © 2023 Adam Erickson, PhD
///
/// 2023-12-01
///

#include <cinttypes>
#include <string>
#include <vector>
#include <unordered_map>
#include <functional>
#include <filesystem>
#include <fstream>
#include <cmath>
#include <cstdio>

///
/// Variables
///

// aliases
using fspath = std::filesystem::path;

// string {word: digit} key-value store
std::unordered_map<std::string, std::string> DIGITS = {
    {"one",   "1"},
    {"two",   "2"},
    {"three", "3"},
    {"four",  "4"},
    {"five",  "5"},
    {"six",   "6"},
    {"seven", "7"},
    {"eight", "8"},
    {"nine",  "9"},
};

// string-int {color: code} key-value store
std::unordered_map<std::string, uint8_t> ANSICODES = {
    {"black",   30},
    {"red",     31},
    {"green",   32},
    {"yellow",  33},
    {"blue",    34},
    {"magenta", 35},
    {"cyan",    36},
    {"white",   37},
    {"default", 39},
};

///
/// Functions
///

// convert char to int.
uint32_t ctoi(char c) {
    return c - '0';  // equivalent to `(uint32_t)c - 48`
}

// check if char is digit. 0-9 is 48-57 in ASCII/UTF-8 decimal.
bool is_digit(char c) {
    uint32_t d = (uint32_t)c;
    return (d > 47 && d < 58) ? true : false;
}

// concatenate two ints to a single int.
uint32_t concat(uint32_t left, uint32_t right) {
    return (left * 10) + right;
}

// concatenate a list of ints of order n to a single int.
// example: [2, 3, 5] -> 235
uint32_t concat(std::vector<uint32_t> ints) {
    uint32_t sum = 0;
    for (int i=0; i<ints.size(); ++i) {
        sum += ints[i] * std::pow(10, i);
    }
    return sum;
}

// find and replace words with digits in-place.
void words_to_digits(std::string& str, size_t i) {
    // variables
    size_t n_key;
    // for char, substring to end, and compare to keys in lookup table
    for (auto& kv : DIGITS) {
        if (str[i] == kv.first[0]) {
            // check key length against char array bounds and then key
            n_key = kv.first.size();
            if (str.substr(i).size() >= n_key && str.substr(i, n_key) == kv.first) {
                str.replace(i, n_key, kv.second);  // replace word chars with digit if match found
            }
        }
    }
    return;
}

// collect sum of left- and right-most digits in string.
uint32_t line_sum(std::string line, bool REPLACE, bool VERBOSE) {
    // variables
    // NOTE: supports full UTF-8 (uint32) encoding, but ASCII (uint8) is the same up to 128.
    size_t n = line.size();  // string n-char
    size_t i = 0;            // left iterator start position
    size_t j = n-1;          // right iterator start position
    char c;                  // char
    std::string word;        // char accumulation for dictionary mapping
    uint32_t left, right;    // left and right digits
    uint32_t digits = 0;   // left-right digit concatenation
    // increment right from left-most char
    for (; i<n; ++i) {
        if (REPLACE) {
            words_to_digits(line, i);
            n = line.size();
            j = n-1;
        }
        c = line[i];
        // REPLACE
        // if digit is found, record value and increment left from right-most char
        if (std::isdigit(c)) {
            (VERBOSE) && printf("left:   %c\n", c);
            left = ctoi(c);
            // decrement from right-most char until j=i (duplicate if single number)
            for (; j+1>i; --j) {
                if (REPLACE) { words_to_digits(line, j); }
                c = line[j];
                // if digit is found on right, record it, calculate sum, return value
                if (std::isdigit(c)) {
                    (VERBOSE) && printf("right:  %c\n", c);
                    right = ctoi(c);
                    digits = concat(left, right);
                    (VERBOSE) && printf("concat: %d\n", digits);
                    return digits;
                }
            }
        }
    }
    return digits;
}

// colorize string using ANSI codes and escape sequences
// -----------------------------------------------------------------------------
// Color 	Example 	Text 	Background 	Bright Text 	Bright Background
// -----------------------------------------------------------------------------
// Black 	Black 	    30 	    40 	        90 	            100
// Red 	    Red 	    31 	    41 	        91 	            101
// Green 	Green 	    32 	    42 	        92 	            102
// Yellow 	Yellow 	    33 	    43 	        93 	            103
// Blue     Blue 	    34 	    44 	        94 	            104
// Magenta 	Magenta 	35 	    45 	        95 	            105
// Cyan     Cyan 	    36 	    46 	        96 	            106
// White 	White 	    37 	    47 	        97 	            107
// Default 		        39 	    49 	        99 	            109
// -----------------------------------------------------------------------------
std::string colorize(std::string str, std::string color="default", bool back=false, bool bright=false) {
    uint8_t code = ANSICODES[color];
    if (back)   code += 10;
    if (bright) code += 60;
    std::string code_string = std::to_string(code);
    return "\033[" + code_string + "m" + str + "\033[0m";
}

// Print the problem statement.
void problem() {
    fspath filepath = "../problem.txt";
    std::ifstream file = filepath;
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string statement = buffer.str();
    printf("\n%s\n", colorize(statement, "cyan", false, false).c_str());
}

// help message.
void help() {
    const std::string MESSAGE = R"(
Usage: main[EXE] [OPTIONS] [--input <PATH>] [--replace <NAME>] [--verbose] [--problem] [--help]

Return the solution to Advent of Code 2023-12-01: Trebuchet.

Options:
  -i, --input <PATH>    Path to the input file. [default: '.\']
  -v, --verbose         Enable verbose message output.
  -p, --problem         Print problem statement.
  -h, --help            Print this help message and exit.

Examples:
  $ ./main --problem
  $ ./main --replace iterator
    )";

    printf("%s\n", colorize(MESSAGE, "cyan").c_str());
    return;
}

bool banner() {
    const std::string BANNER = R"(
    Advent of Code 2023
    December 01: Trebuchet!
                             .`.
                            / `.`.
     ______________________/____`_`____________________________
    / .''.  _        _           _          _           __..--->.
    \ '()'       _       .''.        _       ____...---'       .'
     |_||______.`.__  .' .'______......-----'                 /
      .||-||-./ `.`.' .'   \/_/  `./   /`.`.                .'
    .'_||__.'/ (O)`.`.    \/_/     `./   /`.`.             /
    |_ -  _|/\     /`.`. \/_/        `./   /`.`.          /
    | - _  /\   ./   /`.`. /___________`./   /`.`._     .'
    '-----/\  \/ `./   /`.`._____________`._____` .|   /
         /\  \/_/  `./   /`.`.________________.'.'.' .'
        /\  \/_/   .-`./   /`.`.---------.''.-----.-'
       /\  \/_/  .'~ _ `./   /`.`. _ ~   '..'`._.'
    .'/\  \/_/  '--------`./   /`.`.-----------' 
  .' /\  \/ /______________`./   /`.`..'.'.'
.'__/____/___________________`._____` .'.'
|____________________________________|.'
    )";
    printf("%s\n", colorize(BANNER, "cyan", false, false).c_str());
    return 1;
}

// help funtion to check if argument exists.
bool arg_exists(char* argv[], int i) {
    return (argv[i] != NULL) ? true : false;
}

///
/// Main
///

int main(int argc, char* argv[]) {

    // parse command-line arguments
    fspath filepath = "../input.txt";
    bool REPLACE = false;  // default
    bool VERBOSE = false;  // default

    for (int i=1; i<argc; ++i) {
        std::string arg = argv[i];
        //printf("arg: %s\n", arg.c_str());
        if (arg == "--input" || arg == "-i") {
            if (arg_exists(argv, i+1)) {
                filepath = argv[i+1];
                ++i;
            } else {
                printf("Argument not found: filepath\n");
                return 1;
            }
        } else if (arg == "--replace" || arg == "-r") {
            REPLACE = true;
        } else if (arg == "--verbose" || arg == "-v") {
            VERBOSE = true;
        } else if (arg == "--problem" || arg == "-p") {
            problem();
            return 0;
        } else if (arg == "--help" || arg == "-h") {
            help();
            return 0;
        } else {
            printf("Error: Invalid command-line argument. Exiting program.\n");
            return 1;
        }
    }
    (VERBOSE) && banner();
    (VERBOSE) && (REPLACE) && printf("Replacing string words with digits.\n");

    // variables
    std::ifstream file = filepath;
    std::string line;
    std::string line_digits;
    uint32_t sum_line;
    uint32_t sum_total = 0;
    
    // get sum for each line and add it to total sum
    uint32_t counter = 0;
    if (file.is_open()) {
        while (std::getline(file, line)) {
            counter++;
            if (VERBOSE) {
                printf("#: %d\n", counter);
                printf("line:   %s\n", line.c_str());
            }
            (VERBOSE) && printf("digits: %s\n", line.c_str());
            sum_line   = line_sum(line, REPLACE, VERBOSE);
            sum_total += sum_line;
        }
        file.close();
    }
    printf("%d\n", sum_total);
    return 0;
}

// 56322 < sum < 56805