module Utilities

export colorize, printfile

using Printf
using Base: ImmutableDict

ANSICODES::ImmutableDict{String,Int} = ImmutableDict(
    "black" => 30,
    "red" => 31,
    "green" => 32,
    "yellow" => 33,
    "blue" => 34,
    "magenta" => 35,
    "cyan" => 36,
    "white" => 37,
    "default" => 39
)

###
### Functions
###

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
    back && (code += 10)
    bright && (code += 60)
    code_string::String = string(code)
    return "\033[" * code_string * "m" * text * "\033[0m"
end

# helper methods to print text files to string
function printfile(filepath::String)
    :Nothing
    filepath = abspath(filepath)
    !isfile(filepath) && error("File not found: $(filepath)")
    text::String = read(filepath, String)
    @printf("%s\n", colorize(text, "cyan"))
    return
end

end