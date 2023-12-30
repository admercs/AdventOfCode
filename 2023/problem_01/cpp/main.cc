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
    {"zero",  "0"},
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

// replace all digit words with integers in-place
void words_to_digits(std::string& str) {
    // variables
    size_t n = str.size();
    size_t n_key;
    size_t i;
    // loop over each char, substring to end, and compare to keys in lookup table
    for (i=0; i<n; ++i) {
        for (auto& kv : DIGITS) {
            if (str[i] == kv.first[0]) {
                n_key = kv.first.size();
                // check key length against char array bounds and then key
                if (str.substr(i).size() >= n_key && str.substr(i, n_key) == kv.first) {
                    str.replace(i, n_key, kv.second);  // replace with digit if match found
                    n = str.size();                    // WARNING: update string size after replacement!
                }
            }
        }
    }
    return;
}

// replace all digit words with integers in-place using the standard library
void words_to_digits_std(std::string& str) {
  for (auto& kv : DIGITS) {
    while (str.find(kv.first) != std::string::npos) {
      str.replace(str.find(kv.first), kv.first.size(), kv.second);
    }
  }
  return;
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

// concatenate two ints to a single int.
uint32_t concat(uint32_t left, uint32_t right) {
    return (left * 10) + right;
}

// collect sum of left- and right-most digits in string.
uint32_t line_sum(std::string line, bool VERBOSE) {
    // variables
    // NOTE: we support UTF-8 (uint32) encoding, but digits are the same in ASCII (uint8)
    size_t n = line.size();  // string n-char
    size_t i = 0;            // left iterator start position
    size_t j = n-1;          // right iterator start position
    char c;                  // char
    std::string word;        // char accumulation for dictionary mapping
    uint32_t left, right;    // left and right digits
    uint32_t sum = 0;        // left-right digit sum
    // increment right from left-most char
    for (; i<n; ++i) {
        c = line[i];
        // if digit is found, record value and increment left from right-most char
        if (std::isdigit(c)) {
            (VERBOSE) && printf("left:   %c\n", c);
            left = ctoi(c);
            // decrement from right-most char until j=i (duplicate if single number)
            for (; j+1>i; --j) {
                c = line[j];
                // if digit is found on right, record it, calculate sum, return value
                if (std::isdigit(c)) {
                    (VERBOSE) && printf("right:  %c\n", c);
                    right = ctoi(c);
                    sum = concat(left, right);
                    (VERBOSE) && printf("sum:    %d\n", sum);
                    return sum;
                }
            }
        }
    }
    return sum;
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

// help message.
void help() {
    const std::string message = R"(
    ./main [--replace|-r] [--verbose|-v] [--help|-h]

    Return the solution to Advent of Code 2023-12-01: Trebuchet.

    where:
      -r | --replace    Enable replacement of string words with digits.
      -v | --verbose    Enable verbose message output.
      -h | --help       Print this help message and exit.
    )";
    printf("%s\n", colorize(message, "red").c_str());
    return;
}

///
/// Main
///

int main(int argc, char* argv[]) {

    // parse command-line arguments
    bool REPLACE = false;  // replace string words with digits
    bool VERBOSE = false;  // enable debugging mode

    size_t i;
    for (i=1; i<argc; ++i) {
        std::string arg = argv[i];
        //printf("arg: %s\n", arg.c_str());
        if (arg == "--replace" || arg == "-r") {
            REPLACE = true;
        } else if (arg == "--verbose" || arg == "-v") {
            VERBOSE = true;
        } else if (arg == "--help" || arg == "-h") {
            help();
            return 0;
        } else {
            printf("Error: Invalid command-line argument. Exiting program.\n");
            return 1;
        }
    }
    (VERBOSE) && printf("VERBOSE mode enabled.\n");
    (REPLACE) && printf("REPLACE string words with digits enabled.\n");

    // variables
    std::string filename = "../input.txt";
    fspath filepath      = filename;
    std::ifstream file   = filepath;
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
            if (REPLACE) {
                words_to_digits(line);
            }
            (VERBOSE) && printf("digits: %s\n", line.c_str());
            sum_line   = line_sum(line, VERBOSE);
            sum_total += sum_line;
            (VERBOSE) && printf("total:  %d\n---\n", sum_total);
        }
        file.close();
    }
    printf("%d", sum_total);
    return 0;
}
