""" main.py

Advent of Code 2023, December 1: Trebuchet, Part 1&2
 
Copyright 2023 Adam Erickson, PhD
 
2023-12-01

"""

###
### Imports
###

import os
import argparse

###
### Variables
###

# string {word: digit} key-value store
DIGITS:dict = {
    "zero":  "0",
    "one":   "1",
    "two":   "2",
    "three": "3",
    "four":  "4",
    "five":  "5",
    "six":   "6",
    "seven": "7",
    "eight": "8",
    "nine":  "9"
}

# string-int {color: code} key-value store
ANSICODES:dict = {
    "black":   30,
    "red":     31,
    "green":   32,
    "yellow":  33,
    "blue":    34,
    "magenta": 35,
    "cyan":    36,
    "white":   37,
    "default": 39
}

###
### Functions
###

def ctoi(char:chr) -> int:
    return ord(char) - 48  # same as `int(c - '0') or `int(c)`

def words_to_digits(line:str) -> str:
    # variables
    n:int = len(line)
    # loop over each char, substring to end, and compare to keys in lookup table
    for i in range(0, n, 1):
        for k,v in DIGITS.items():
            if (str[i] == k[0]):
                n_key = len(k)
                # check key length against char array bounds and then key
                if (str.substr(i).size() >= n_key and str.substr(i, n_key) == k):
                    str.replace(i, n_key, v)  # replace with digit if match found
                    n = str.size()            # WARNING: update string size after replacement!
    return None

def line_sum(line:str) -> int:
    # variables
    char:char = '0'
    left:int  = 0
    right:int = 0
    summ:int  = 0
    # loop over chars from left, then right
    for i in range(0, len(line), 1):
        char = line[i]
        if char.isdigit():
            left = int(char)
            for j in range(len(line)-1, i-1, -1):
                char = line[j]
                if char.isdigit():
                    right = int(char)
                    summ = left + right
                    return summ
    return 0

# colorize string using ANSI codes and escape sequences.
# -----------------------------------------------------------------------------
# Color 	Example 	Text 	Background 	Bright Text 	Bright Background
# -----------------------------------------------------------------------------
# Black 	Black 	    30 	    40 	        90 	            100
# Red 	    Red 	    31 	    41 	        91 	            101
# Green 	Green 	    32 	    42 	        92 	            102
# Yellow 	Yellow 	    33 	    43 	        93 	            103
# Blue      Blue 	    34 	    44 	        94 	            104
# Magenta 	Magenta 	35 	    45 	        95 	            105
# Cyan      Cyan 	    36 	    46 	        96 	            106
# White 	White 	    37 	    47 	        97 	            107
# Default 		        39 	    49 	        99 	            109
# -----------------------------------------------------------------------------
def colorize(text:str, color:str="default", back:bool=False, bright:bool=False):
    code:int = ANSICODES[color]
    back   and (code := code + 10)
    bright and (code := code + 60)
    code_string:str = str(code)
    return "\033[" + code_string + "m" + text + "\033[0m"

# help message.
def print_help():
    message:str = r"""
    ./main.py [OPTIONS] [--filepath|-f FILEPATH] [--replace|-r] [--verbose|-v] [--help|-h]

    Return the solution to Advent of Code 2023-12-01: Trebuchet.

    Options:
      -f, --filepath    Path to input file.
      -r, --replace     Enable replacement of string words with digits.
      -v, --verbose     Enable verbose message output.
      -h, --help        Print this help message and exit.
    """
    print("%s\n" % colorize(message, "red"))
    return None

###
### Main
###

if __name__ == '__main__':

    # parse command-line arguments.
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument('-f', '--filepath', default=os.path.abspath(os.path.join("..", "input.txt")))
    parser.add_argument('-r', '--replace',  action='store_true')
    parser.add_argument('-v', '--verbose',  action='store_true')
    parser.add_argument('-h', '--help',     action='store_true')
    args = parser.parse_args()

    # check arguments
    if args.help:
        print_help()
        exit(0)
    args.verbose and print("filepath: %s, replace: %s" % (args.filepath, "true" if args.replace else "false"))
    os.path.isfile(args.filepath) or Exception("File path is invalid.")

    # variables
    i:int = 0
    total:int = 0

    # open file handler and loop over file
    with open(args.filepath) as io:
        for line in io:
            i += 1
            line_strip = line.strip()
            total += line_sum(line_strip)
            if args.verbose:
                print("%s:     %d" % (colorize("i", "green"), i))
                print("line:  %s"  % line_strip)
                print("%s: %d"     % (colorize("total", "red"), total))
    
    print("%d" % total)
