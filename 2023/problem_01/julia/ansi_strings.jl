# ansi_strings.jl

using Base: ImmutableDict

module AnsiStrings

export colorize

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
function colorize(str::String, code::Int=30, back::Bool=false, bright::Bool=false)::String
    back   && (code += 10);
    bright && (code += 60);
    code_string::String = string(code);
    return "\033[" + code_string + "m" + str + "\033[0m";
end

end