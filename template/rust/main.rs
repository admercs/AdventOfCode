//!
//! FILENAME
//!   AdventOfCode/2023/day_01_trubuchet/main.rs
//!
//! DESCRIPTION
//!   December 01: The Trebuchet Problem
//!
//! AUTHOR
//!   Adam Erickson, PhD
//!
//! DATE
//!   2023-12-01
//!
//! NOTES
//!   -
//!

#![allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

///
/// Variables, Data Structures, Types
///

/* CLI arguments */
struct CliArgs {
    input: PathBuf,
    method_sum: String,
    replace: bool,
    verbose: bool,
}
type Args = CliArgs;

///
/// Functions
///

/*
 * Colorize strings using ANSI escape sequences.
 * -------------------------------------------------------------------
 * | Color   | Text | Background | Bright Text  |  Bright Background |
 * -------------------------------------------------------------------
 * | Black   |  30  | 40         | 90           | 100                |
 * | Red     |  31  | 41         | 91           | 101                |
 * | Green   |  32  | 42         | 92           | 102                |
 * | Yellow  |  33  | 43         | 93           | 103                |
 * | Blue    |  34  | 44         | 94           | 104                |
 * | Magenta |  35  | 45         | 95           | 105                |
 * | Cyan    |  36  | 46         | 96           | 106                |
 * | White   |  37  | 47         | 97           | 107                |
 * | Default |  39  | 49         | 99           | 109                |
 * -------------------------------------------------------------------
 */
fn colorize(text: &str, color: &str, bright: bool, back: bool) -> String {
    let mut code: u8 = match color {
        "black" => 30,
        "red" => 31,
        "green" => 32,
        "yellow" => 33,
        "blue" => 34,
        "magenta" => 35,
        "cyan" => 36,
        "white" => 37,
        "default" => 39,
        &_ => 39, // set to default if no match found
    };
    if back {
        code += 10;
    }
    if bright {
        code += 60;
    }
    return format!("\x1b[{}m{}\x1b[0m", code, text);
}

/*
 * Read lines from a file path. Output is wrapped in a Result to allow matching on errors.
 * Returns an Iterator to the Reader of the lines of the file.
 */
fn read_lines<P>(filename: P) -> IoResult<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

/*
 * Print the problem statement.
 */
fn problem() {
    let filepath: PathBuf = Path::new("..").join("problem.txt");
    let statement: String = std::fs::read_to_string(filepath).unwrap().parse().unwrap();
    println!("\n{}\n", colorize(&statement, "cyan", false, false));
}

/*
 * Print the banner.
 */
fn banner() {
    let filepath: PathBuf = Path::new("..").join("banner.txt");
    let banner: String = std::fs::read_to_string(filepath).unwrap().parse().unwrap();
    println!("\n{}\n", colorize(&banner, "cyan", false, false));
}

/*
 * Print help message.
 */
fn help() {
    const HELP: &'static str = r#"
Usage: main[EXE] [OPTIONS] [--input <PATH>] [--sum <NAME>] [--replace] [--verbose] [--problem] [--help]

Return the solution to Advent of Code 2023-12-01: Trebuchet.

Options:
  -i, --input <PATH>        Path to the input file. [default: '.\']
  -s, --sum <NAME>          Line summation method. [options: ["array", "iterator"; default: "array"]
  -r, --replace             Replace words with digits.
  -v, --verbose             Print debugging information.
  -p, --problem             Print problem statement.
  -h, --help                Print this help message.

Examples:
  $ /.main.exe --problem
  $ /.main.exe --sum array --replace
  $ /.main.exe --input .\data\input.txt --sum array --replace
  $ /.main.exe --help
    "#;
    println!("{}", colorize(HELP, "cyan", false, false));
}

/*
 * Parse command-line arguments.
 */
fn parse_args() -> Args {
    // defaults for variables that we store in CliArgs
    let mut input: PathBuf = Path::new("..").join("input.txt");
    let mut method_sum: String = String::from("array");
    let mut replace: bool = false;
    let mut verbose: bool = false;
    // loop over CLI arguments
    let mut cli_args: std::iter::Skip<env::Args> = env::args().skip(1);
    while let Some(arg) = cli_args.next() {
        match &arg[..] {
            "-i" | "--input" => {
                if let Some(arg_input) = cli_args.next() {
                    input = PathBuf::from(arg_input);
                }
            }
            "-s" | "--sum" => {
                if let Some(arg_method_sum) = cli_args.next() {
                    method_sum = String::from(arg_method_sum);
                }
            }
            "-r" | "--replace" => {
                replace = true;
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "-p" | "--problem" => {
                problem();
                std::process::exit(0);
            }
            "-h" | "--help" => {
                help();
                std::process::exit(0);
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument: {}", arg);
                } else {
                    println!("Unknown positional argument: {}", arg);
                }
            }
        }
    }
    return CliArgs {
        input,
        method_sum,
        replace,
        verbose,
    };
}

///
/// Main
///

fn main() -> Result<(), &'static str> {
    // assertions

    // parse command-line arguments
    let args: Args = parse_args();
    if args.verbose {
        banner();
        println!(
            "input: {}, method-sum: {}, method-replace: {}",
            args.input.display(),
            args.method_sum,
            args.replace
        );
    }

    // variables
    let mut i: usize; // line counter
    let mut sum: u32 = 0; // rolling sum
    let mut line_sum; // line_sum function
    let mut line_string: String;

    // open `input.txt` file reader buffer
    if args.verbose {
        println!("Opening file from path: {}", &args.input.display());
    }
    let file = File::open(&args.input).expect("Failed to open file.");
    let reader = BufReader::new(file);

    // read input file line-by-line and each char from left to right
    i = 0;
    for line in reader.lines() {
        i += 1;
        if args.verbose {
            println!(
                "#: {}",
                colorize(i.to_string().as_str(), "cyan", true, false)
            );
        }
        // do something
    }
    if args.verbose {
        println!("{}: {}", colorize("total", "red", true, false), sum);
    }
    // If no error, return `sum`
    println!("{}", sum);
    Ok(())
}
