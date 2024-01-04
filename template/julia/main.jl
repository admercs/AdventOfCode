#= main.jl

Advent of Code 2023, December 1: Trebuchet, Part 1&2

Copyright 2023 Adam Erickson, PhD

2023-12-01

=#

###
### Imports
###

using Core, Base, Printf

include("Utilities.jl")
using .Utilities: colorize, printfile

include("SimpleArgParse.jl")
using .SimpleArgParse: ArgumentParser, add_argument, parse_args, exists, get, set

###
### Variables
###

###
### Functions
###

banner()::Nothing  = printfile(joinpath("..", "banner.txt"))
problem()::Nothing = printfile(joinpath("..", "problem.txt"))

# help message.
function help()
    :Nothing
    message::String = raw"
    ./main.jl [--input] [--verbose] [--problem] [--help]

    Return solution to Advent of Code problem.

    Options:
      -i, --input <PATH>    Enable replacement of string words with digits.
      -v, --verbose         Enable verbose message output.
      -p, --problem         Print the problem statement.
      -h, --help            Print this help message.

    Examples:
      $ julia main.jl --input \"dir/file.txt\" --verbose
      $ julia main.jl --help
    "
    @printf("%s\n", colorize(message, "cyan"))
    return nothing
end

###
### Main
###

function main()
    :Int

    args::ArgumentParser = ArgumentParser(description="Advent of Code 2023")
    args = add_argument(args, "-i", "--input",   type=String, default=abspath(joinpath("..", "input.txt")))
    args = add_argument(args, "-v", "--verbose", type=Bool,   default=false)
    args = add_argument(args, "-p", "--problem", type=Bool,   default=false)
    args = add_argument(args, "-h", "--help",    type=Bool,   default=false)
    args = parse_args(args)

    #exists(args, "input")   && @printf("arg `input` exists\n")
    #exists(args, "verbose") && @printf("arg `verbose` exists\n")
    #exists(args, 'v')       && @printf("key `v` exists\n\n")

    #@printf("input: %s\n", get(args, "input"))
    get(args, "verbose") && banner()
    get(args, "problem") && problem()
    get(args, "help")    && help()

    # variables
    solution::Int32 = 0

    # solution
    get(args, "verbose") && println("Solution:")
    @printf("%d", solution)
    return 0
end

main()
