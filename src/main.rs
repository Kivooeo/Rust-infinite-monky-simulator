// For time of lastest most succesful attemp
use chrono;
// For beautiful console output
use colored::*;
// For generating random char
use rand::{thread_rng, Rng};
// For open file with append mode
use std::fs::OpenOptions;
// Here is alphabet, all symbols that contains in string that you want to generate
const CHARS: &'static str = "ni{}(maf)";
// Path to log file
const PATH: &str = "./ascvbnkl";
// the phrase you want to give the monkey so that it can guess it
const ANSWER: &str = "fn main(){}";
// Attemps counter
static mut C: u128 = 0;
// It easily can be situation that attemps counter reach overflow
// So for continue counting attemps i made up this thing
static mut OVERFLOW_TIMES: u128 = 0;
// Maximum value that monky could find
static mut MAX: u8 = 0;

fn main() {
    // println!("{}", create_alphabet("fn main(){}"));
    // Start function, Do not ask me about this.
    _start();
}

// UNSAFE SAFETY FOR ALL UNSAFE BLOCKS BELOW:
// This just reading or changing value of static mut in sync way
// So there is no way how this can be really unsafe

// Main function, Do not ask me about this.
fn _start() {
    // This is will store chars before monky make typo
    let mut answer = String::new();
    // This counting how much chars monky guess right
    // TODO: USE answer.len() instead???
    let mut count = 0;
    // Current time? Nice type rust-analyzer really gj
    let mut time: chrono::prelude::DateTime<chrono::prelude::Local> = chrono::offset::Local::now();
    loop {
        // First step, generate a char.
        let c = gen_char();
        // Second step, check correct char or not
        if check_char(c, count) {
            // If it correct then we push
            answer.push(c);
            // And checking len of string
            // If it more than our maximum that monky find ever
            // We change it to new max
            if answer.len() as u8 > unsafe { MAX } {
                time = chrono::offset::Local::now();
                unsafe { MAX = answer.len() as u8 }
                // And adding to our log info about new maximum reached
                add_try();
            }
            // Go to next char position
            count += 1;
            // Checking reach we end or not
            if answer.len() == ANSWER.len() {
                break;
            }
            // Othwerwise our attemp is failed so we
        } else {
            // Clear string
            answer.clear();
            // And reset position counter
            count = 0;
            // And print this attemp
            // In way below:
            // attemps: 1 ::: max: pharse ::: time: 29/12/2023 02:35:25

            unsafe {
                println!(
                    "attemps: {} ::: max: {} ::: time: {}",
                    C.to_string().blue().italic(),
                    &ANSWER[0..MAX as usize].green().bold(),
                    time.format("%d/%m/%Y %H:%M:%S")
                        .to_string()
                        .yellow()
                        .italic()
                        .bold(),
                );
                // Also check if our attemp counter reached overflow
                // If yes then we increment overflow counter and reset main counter
                // TODO: Checked_add???
                if C == u128::MAX - 1 {
                    OVERFLOW_TIMES += 1;
                    C = 0;
                }
                C += 1
            }
        }
    }
}

// Random char
fn gen_char() -> char {
    CHARS
        .chars()
        .nth(thread_rng().gen_range(0..CHARS.len()))
        .unwrap()
}

// Check if char that we randomed with position in position in our final answer
fn check_char(ch: char, c: usize) -> bool {
    ANSWER.chars().nth(c).unwrap() == ch
}

// Importing write? why here?
// TODO: Replace it to top
use std::io::Write;

// Function to write into log, info about lastest most succesful try that monky do
fn add_try() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(PATH)
        .unwrap();
    unsafe {
        let temp = MAX as usize;

        if let Err(e) = writeln!(
            file,
            "[Times: {:>2}] ::: [Attemp at this time: {:<39}] ::: [Current text: {:<temp$}] ::: [Current time: {}]",
            OVERFLOW_TIMES, C, &ANSWER[0..MAX as usize], chrono::offset::Local::now().format("%d/%m/%Y %H:%M:%S")
            .to_string()
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn create_alphabet(x: &str) -> String {
    std::collections::HashSet::<_>::from_iter(x.chars())
        .iter()
        .collect()
}
