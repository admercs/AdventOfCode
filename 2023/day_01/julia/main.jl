#= main.jl

Advent of Code 2023, December 1: Trebuchet, Part 1&2
 
Copyright 2023 Adam Erickson, PhD
 
2023-12-01

=#

###
### Imports
###

using Core, Base, Printf
using Base: ImmutableDict

###
### Variables
###

DEBUG::Bool = false

DIGITS::ImmutableDict{String, Char} = ImmutableDict(
    "one"   => '1',
    "two"   => '2',
    "three" => '3',
    "four"  => '4',
    "five"  => '5',
    "six"   => '6',
    "seven" => '7',
    "eight" => '8',
    "nine"  => '9'
)

ANSICODES::ImmutableDict{String, Int} = ImmutableDict(
    "black"   => 30,
    "red"     => 31,
    "green"   => 32,
    "yellow"  => 33,
    "blue"    => 34,
    "magenta" => 35,
    "cyan"    => 36,
    "white"   => 37,
    "default" => 39
)

###
### Functions
###

function ctoi(char::Char)::Int
    return Int(char) - 48  # same as `Int(c - '0')`
end

function word_to_digit(line::String, i::Int)::String
    # loop over each char, substring to end, and compare to keys in lookup table
    for (k,v) in DIGITS
        if line[i] == k[1]
            # check key length against char array bounds and then key
            n_key = length(k)
            if length(line[i:end]) >= n_key && line[i:i+n_key-1] == k
                line = line[1:i-1] * v * line[i+n_key:end]  # replace with digit if match found
            end
        end
    end
    return line
end

function line_sum(line::String, replace::Bool)::Int
    # variables
    n::Int      = lastindex(line)
    left::Int   = 0
    right::Int  = 0
    concat::Int = 0
    char::Char  = Char(48)  # 0
    # loop over chars from left, then right
    for i = firstindex(line):1:n
        if replace
            line = word_to_digit(line, i)
            n = lastindex(line)
        end
        char = line[i]
        if isdigit(char)
            left = ctoi(char)
            for j = lastindex(line):-1:i
                if replace
                    line = word_to_digit(line, j)
                end
                char = line[j]
                if isdigit(char)
                    right = ctoi(char)
                    concat = (left * 10) + right
                    return concat
                end
            end
        end
    end
    return 0
end

function file_sum(filepath::String, replace::Bool)::Int
    isfile(filepath) || throw(ErrorException("File path is invalid."))
    # variables
    total::Int = 0
    # open and loop over file
    open(filepath) do io
        while ! eof(io)
            line = readline(io)
            DEBUG && println(line)
            total += line_sum(line, replace)
        end
    end
    return total
end

# colorize string using ANSI codes and escape sequences
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
function colorize(text::String, color::String="default", back::Bool=false, bright::Bool=false)::String
    code::Int8 = ANSICODES[color]
    back   && (code += 10);
    bright && (code += 60);
    code_string::String = string(code);
    return "\033[" * code_string * "m" * text * "\033[0m";
end

# help message.
function help()
    message::String = raw"
    ./main.jl [--replace|-r] [--verbose|-v] [--help|-h]

    Return the solution to Advent of Code 2023-12-01: Trebuchet.

    where:
      -r | --replace    Enable replacement of string words with digits.
      -v | --verbose    Enable verbose message output.
      -h | --help       Print this help message and exit.
    ";
    @printf("%s\n", colorize(message, "red"))
    return nothing
end

###
### Main
###

function main()::Int
    # variables
    REPLACE::Bool = false
    VERBOSE:Bool = false

    # parse command-line arguments
    for arg in ARGS
        if arg == "--replace" || arg == "-r"
            REPLACE = true
        elseif arg == "--verbose" || arg == "-v"
            VERBOSE = true
        elseif arg == "--help" || arg == "-h"
            help()
            return 0
        else
            @printf("Error: Invalid command-line argument. Exiting program.\n")
            return 1
        end
    end

    # processing data
    filepath = abspath(joinpath("..", "input.txt"))
    total = file_sum(filepath, REPLACE)
    @printf("%d", total)
    return 0
end

main()
