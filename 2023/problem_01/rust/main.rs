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

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

/*
 * Crates
 *
 * Aho-Corasick algorithm for time- and space-optimal string replacement.
 * Time complexity: O(n + m + z) where:
 *   n = string length
 *   m = number of characters in all words
 *   z = number of word occurrences
 * Intuitively, the algorithm works by constructing a fully connected trie of all search words
 * before walking the trie for each character in the string.
 */
use aho_corasick::AhoCorasick;

///
/// Variables, Data Structures, Types
///

/* CLI arguments */
struct CliArgs {
    input: PathBuf,
    method_sum: String,
    method_replace: String,
    verbose: bool,
}
type Args = CliArgs;

/*
 * Digit string-to-byte key-value store.
 */
static WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static DIGITS: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57]; // ASCII/UTF-8 byte encoding

///
/// Functions
///

/*
 * Zip two string arrays into a hashmap.
 */
#[allow(dead_code)]
fn generate_digits_hashmap() -> HashMap<String, u8> {
    let mut map: HashMap<String, u8> = HashMap::new();
    for (k, v) in WORDS.into_iter().zip(DIGITS.into_iter()) {
        map.insert(String::from(k), v);
    }
    return map;
}

/*
 * Get digit char for string. Approximate HashMap.
 */
#[allow(dead_code)]
fn get_digit_char(word: &str) -> char {
    let digit: char = match word {
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        &_ => ' ', // default if no match found
    };
    return digit;
}

/*
 * Get digit byte for byte-array. Approximate HashMap.
 */
#[allow(dead_code)]
fn get_digit_u8(word: &[u8]) -> u8 {
    let digit: u8 = match word {
        [b'z', b'e', b'r', b'o'] => 0,
        [b'o', b'n', b'e'] => 1,
        [b't', b'w', b'o'] => 2,
        [b't', b'h', b'r', b'e', b'e'] => 3,
        [b'f', b'o', b'u', b'r'] => 4,
        [b'f', b'i', b'v', b'e'] => 5,
        [b's', b'i', b'x'] => 6,
        [b's', b'e', b'v', b'e', b'n'] => 7,
        [b'e', b'i', b'g', b'h', b't'] => 8,
        [b'n', b'i', b'n', b'e'] => 9,
        _ => 0, // default value if no match found
    };
    return digit;
}

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
#[allow(dead_code)]
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
 * Encode u8 to char (ASCII/UTF-8).
 */
#[allow(dead_code)]
fn u8_to_char(bytes: u8) -> char {
    let d: u32 = u32::from(bytes);
    let c: char = char::from_u32(d).unwrap();
    return c;
}

/*
 * Decode char (ASCII) to u8.
 */
#[allow(dead_code)]
fn char_to_u8(c: char) -> u8 {
    let u: u8 = c as u8;
    return u - 48; // same as char - '0'
}

/*
 * Decode char (ASCII/UTF-8) to u32.
 */
#[allow(dead_code)]
fn char_to_u32(c: char) -> u32 {
    let u: u32 = u32::from(c);
    return u - 48; // same as char - '0'
}

/*
 * Check if byte array is valid ASCII/UTF-8 codes.
 */
#[allow(dead_code)]
fn is_utf8_bytes(data: &[u8]) -> bool {
    match std::str::from_utf8(&data[0..std::cmp::min(data.len(), 4)]) {
        Ok(s) => s.chars().next().is_some(),
        Err(e) => e.valid_up_to() > 0,
    }
}

/*
 * Determine whether byte is a valid ASCII digit in the range 48-57.
 */
#[allow(dead_code)]
fn is_utf8_byte(byte: u8) -> bool {
    return if byte > 47 && byte < 58 { true } else { false };
}

/*
 * Read lines from a file path. Output is wrapped in a Result to allow matching on errors.
 * Returns an Iterator to the Reader of the lines of the file.
 */
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> IoResult<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

/*
 * Loop over line string (char array) and return the sum of outermost digits.
 * We do not use mutability on left or right, but the compiler complains anyway.
 * Convert string to bytes and loop over indices (the C way) not iterators (the Rust way).
 */
#[allow(unused_mut)]
#[allow(dead_code)]
fn line_sum_array(line: String, replace: bool, verbose: bool) -> u32 {
    // variables
    let n: usize; // line string length
    let mut c: char; // char
    let mut left: u32; // left digit value
    let mut right: u32; // right digit value
    let mut sum: u32 = 0; // sum of first and last digit chars
    let mut line_bytes: Vec<u8> = Vec::from(line.as_bytes());
    if verbose {
        println!("line: {}", std::str::from_utf8(&line_bytes).unwrap());
    }
    // loop over the byte array and collect digits of radix 10 (ASCII numbers)
    n = line_bytes.len();
    let mut j: usize = 0;
    let mut k: usize = n - 1;
    while j < n {
        // check if j is start of digit word; if true, update line_bytes[j] and j in-place
        if replace {
            words_to_digits_array(&mut line_bytes, j, verbose);
        }
        //if verbose { println!("j: {}", j); }
        c = u8_to_char(line_bytes[j]);
        //if verbose { println!("c: {}", c); }
        if c.is_digit(10) {
            left = char_to_u32(c);
            if verbose {
                println!("{}: {}", "left", c);
            }
            while k >= j {
                // check if k is start of digit word; if true, update line_bytes[k] and k in-place
                if replace {
                    words_to_digits_array(&mut line_bytes, k, verbose);
                }
                //println!("k: {}", k);
                c = u8_to_char(line_bytes[k]);
                //println!("c: {}", c);
                if c.is_digit(10) {
                    right = char_to_u32(c);
                    if verbose {
                        println!("{}: {}", "right", c);
                    }
                    sum = left + right;
                    if verbose {
                        println!("{} + {} = {}", left, right, sum);
                    }
                    return sum;
                }
                k -= 1;
            }
        }
        j += 1;
    }
    return sum;
}

/*
 * Idiomatic Rusty variant of `line_sum` function. For comparison of readability and performance.
 */
#[allow(unused_assignments)]
#[allow(dead_code)]
fn line_sum_iterator(line: String, _replace: bool, verbose: bool) -> u32 {
    // variables
    let mut ch: Option<char>;
    let mut left: u32 = 0;
    let mut right: u32 = 0;
    let mut sum: u32 = 0; // sum of first and last digit chars
    if verbose {
        println!("line: {}", line);
    }
    let mut chars: std::str::Chars<'_> = line.chars();
    // loop over chars from left and then from right if digit is found
    loop {
        ch = chars.next(); // pop value from left
        if ch.is_none() {
            break;
        }
        if ch.unwrap().is_digit(10) {
            // if character is a valid ASCII/UTF-8 digit
            left = char_to_u32(ch.unwrap());
            if verbose {
                println!("left: {}", left);
            }
            loop {
                ch = chars.next_back(); // pop value from right
                if ch.is_none() {
                    break;
                }
                if ch.unwrap().is_digit(10) {
                    right = char_to_u32(ch.unwrap());
                    if verbose {
                        println!("right: {}", right);
                    }
                    sum = left + right;
                    if verbose {
                        println!("{} + {} = {}", left, right, sum);
                    }
                    return sum;
                }
            }
        }
    }
    if right == 0 {
        sum = left + left;
        if verbose {
            println!("{} + {} = {}", left, left, sum);
        }
    } else {
        sum = left + right;
        if verbose {
            println!("{} + {} = {}", left, right, sum);
        }
    }
    return sum;
}

/*
 * Higher-order function to parametrically select which line sum method to apply.
 * Options: [ line_sum_array, line_sum_iterator ]
 */
#[allow(dead_code)]
fn line_sum_fn(f: impl Fn(String, bool, bool) -> u32) -> impl Fn(String, bool, bool) -> u32 {
    return f; // f returns a function, f() returns a callback
}

/*
 * Map function name string to function pointer.
 */
#[allow(dead_code)]
fn line_sum_fn_str(name: &str) -> impl Fn(String, bool, bool) -> u32 {
    let f: fn(String, bool, bool) -> u32 = match name {
        "array" => line_sum_array,
        "iterator" => line_sum_iterator,
        _ => unimplemented!(),
    };
    return f;
}

/*
 * Find and replace words with digits from the zeroth index. The outer function increments the index.
 * If char byte is start of digit string bytes, convert to decimal char and increment index by string length.
 */
#[allow(dead_code)]
fn words_to_digits_array(line_bytes: &mut Vec<u8>, index: usize, verbose: bool) -> () {
    // variables
    let mut word_bytes: &[u8]; // word byte array
    let mut word_len: usize; // word length
    let mut substring: &[u8]; // subarray from index to word end
    let mut end: usize; // end index
    for (i, word) in WORDS.iter().enumerate() {
        // loop over each key-value pair and replace words with digits. WORDS and DIGITS are equal length.
        word_bytes = word.as_bytes();
        word_len = word_bytes.len();
        end = index + word_len - 1;
        // bounds check
        if end > line_bytes.len() - 1 {
            continue;
        }
        substring = &line_bytes[index..=end]; // slice from the starting index to n-char
                                              //if verbose {
                                              //    println!(
                                              //        "string: {}, index: {}, end: {}, length: {}, substring: {}, word: {}",
                                              //        std::str::from_utf8(&line_bytes).unwrap(),
                                              //        index,
                                              //        end,
                                              //        word_len,
                                              //        std::str::from_utf8(&substring).unwrap(),
                                              //        word
                                              //    );
                                              //}
        if substring == word_bytes {
            line_bytes[index] = DIGITS[i];
            if verbose {
                println!("updated: {}", std::str::from_utf8(&line_bytes).unwrap());
            }
            break;
        }
    }
    return ();
}

/*
 * Find and replace integer words with digits. Idiomatic built-in method.
 */
#[allow(dead_code)]
fn words_to_digits_builtin(line: String, verbose: bool) -> String {
    if verbose {
        println!("line: {}", line);
    }
    let mut result: String = String::from("");
    // loop over each key-value pair and replace words with digits
    let kv: HashMap<String, u8> = generate_digits_hashmap();
    for (k, v) in kv {
        result = line.replace(&k, std::str::from_utf8(&[v]).unwrap());
    }
    return result;
}

/*
 * Find and replace integer words with digits. Aho-Corasick method from crate.
 */
#[allow(dead_code)]
fn words_to_digits_ac(line: String, verbose: bool) -> String {
    if verbose {
        println!("line: {}", line);
    }
    let mut result: Vec<u8> = vec![];
    let ac: AhoCorasick = AhoCorasick::new(WORDS).unwrap();
    ac.try_stream_replace_all(line.as_bytes(), &mut result, &[DIGITS])
        .unwrap();
    return String::from_utf8_lossy(&result).to_string();
}

/*
 * Higher-order function to parametrically select which line sum method to apply.
 * Options: [ line_sum_array, line_sum_iterator ]
 */
#[allow(dead_code)]
fn words_to_digits_fn(f: impl Fn(String, bool) -> String) -> impl Fn(String, bool) -> String {
    return f; // f returns a function, f() returns a callback
}

/*
 * Map function name string to function pointer.
 */
#[allow(dead_code)]
fn words_to_digits_fn_str(name: &str) -> impl Fn(String, bool) -> String {
    let f: fn(String, bool) -> String = match name {
        "builtin" => words_to_digits_builtin,
        "ac" => words_to_digits_ac,
        _ => unimplemented!(),
    };
    return f;
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
 * Print help message.
 */
fn help() {
    const HELP: &'static str = r#"
An Advent of Code problem and solution generator implemented in Rust.

Usage: main[EXE] [OPTIONS] [--input <PATH>] [--method <NAME>] [--verbose] [--problem] [--help]

Options:
  -i, --input <PATH>        Path to the input file. [default: '.\']
  -s, --sum <NAME>          Line summation method. [options: ["array", "iterator"; default: "array"]
  -r, --replace <NAME>      Word-to-digit method. [options: ["array", "builtin", "ac"; default: ""]
  -v, --verbose             Print debugging information.
  -p, --problem             Print problem statement.
  -h, --help                Print this help message.

Examples:
  $ /.main.exe --problem
  $ /.main.exe --sum array --replace array
  $ /.main.exe --input .\data\input.txt --sum array --replace array
  $ /.main.exe --help
    "#;
    println!("{}", colorize(HELP, "cyan", false, false));
}

fn banner() {
    const BANNER: &'static str = r#"
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
    "#;
    println!("{}", colorize(BANNER, "cyan", false, false));
}

/*
 * Parse command-line arguments.
 */
fn parse_args() -> Args {
    // defaults for variables that we store in CliArgs
    let mut input: PathBuf = Path::new("..").join("input.txt");
    let mut method_sum: String = String::from("array");
    let mut method_replace: String = String::from("");
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
                if let Some(arg_method_replace) = cli_args.next() {
                    method_replace = String::from(arg_method_replace);
                }
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
        method_replace,
        verbose,
    };
}

///
/// Main
///

fn main() -> Result<(), &'static str> {
    banner();
    // validate variable definitions
    assert!(WORDS.len() == DIGITS.len());

    // parse command-line arguments
    let args: Args = parse_args();
    if args.verbose {
        println!(
            "input: {}, method-sum: {}, method-replace: {}",
            args.input.display(),
            args.method_sum,
            args.method_replace
        );
    }

    // switch to enable/disable replacing words with digits
    let enable_replace: bool = if args.method_replace == "" {
        false
    } else {
        true
    };

    // variables
    let mut i: usize; // line counter
    let mut sum: u32 = 0; // rolling sum
    let mut line_sum; // line_sum function
    let mut line_string: String;
    let mut words_to_digits;

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
        line_sum = line_sum_fn_str(&args.method_sum);
        line_string = String::from(line.unwrap().as_str());
        if !line_string.is_ascii() {
            println!("Warning: String contains invalid ASCII: {}", line_string);
        }
        // treat array-based methods differently.
        if args.method_sum == "array" {
            sum += line_sum(line_string, enable_replace, args.verbose);
        } else if enable_replace {
            words_to_digits = words_to_digits_fn_str(&args.method_replace);
            line_string = words_to_digits(line_string, true);
            sum += line_sum(line_string, enable_replace, args.verbose);
        } else {
            sum += line_sum(line_string, enable_replace, args.verbose);
        }
        if args.verbose {
            println!("{}: {}", "sum", sum);
        }
    }
    if args.verbose {
        println!("{}: {}", colorize("total", "red", true, false), sum);
    }
    // If no error, return `sum`
    println!("{}", sum);
    Ok(())
}
