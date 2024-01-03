/// main.cc
///
/// Advent of Code 2023, Day 02: Cube Conundrum
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
#include <cstdio>

///
/// Variables
///

// aliases
using fspath = std::filesystem::path;

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

// print the problem statement.
void problem() {
    fspath filepath = "../problem.txt";
    std::ifstream file = filepath;
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string PROBLEM = buffer.str();
    printf("\n%s\n", colorize(PROBLEM, "cyan", false, false).c_str());
}

// print the banner.
void banner() {
    fspath filepath = "../banner.txt";
    std::ifstream file = filepath;
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string BANNER = buffer.str();
    printf("\n%s\n", colorize(BANNER, "cyan", false, false).c_str());
    return;
}

// help funtion to check if argument exists.
bool arg_exists(char* argv[], int i) {
    return (argv[i] != NULL) ? true : false;
}

// help message.
void help() {
    const std::string MESSAGE = R"(
Usage: main[EXE] [OPTIONS] [--input <PATH>] [--verbose] [--problem] [--help]

Return the solution to Advent of Code 2023-12-01: Trebuchet.

Options:
  -i, --input <PATH>    Path to the input file. [default: '.\']
  -v, --verbose         Enable verbose message output.
  -p, --problem         Print problem statement.
  -h, --help            Print this help message and exit.

Examples:
  $ ./main --problem
  $ ./main
    )";
    printf("%s\n", colorize(MESSAGE, "cyan").c_str());
    return;
}

///
/// Main
///

int main(int argc, char* argv[]) {

    // defaults
    fspath filepath = "../input.txt";
    bool VERBOSE = false;

    // parse command-line arguments
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
    if (VERBOSE) banner();

    // variables
    std::ifstream file = filepath;
    std::string line;
    uint32_t solution = 0;
    
    // iterate over lines and characters in file
    if (file.is_open()) {
        while (std::getline(file, line)) {
            (VERBOSE) && printf("\n%s\n", line.c_str());
            // SOLUTION
        }
        file.close();
    }

    (VERBOSE) && printf("\n");
    printf("%d\n", solution);
    return 0;
}
