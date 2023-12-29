use chrono;
use colored::*;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;

const CHARS: &'static str = "ni{}(maf)";
const PATH: &str = "./ascvbnkl";
const ANSWER: &str = "fn main(){}";

static mut C: u128 = 0;
static mut OVERFLOW_TIMES: u128 = 0;
static mut MAX: u8 = 0;

fn main() {
    let mut answer = String::new();
    let mut count = 0;
    let mut time: chrono::prelude::DateTime<chrono::prelude::Local> = chrono::offset::Local::now();
    loop {
        let c = gen_char();
        if check_char(c, count) {
            answer.push(c);
            if answer.len() as u8 > unsafe { MAX } {
                time = chrono::offset::Local::now();
                unsafe { MAX = answer.len() as u8 }
                add_try();
            }
            count += 1;
            if answer.len() == ANSWER.len() {
                break;
            }
        } else {
            answer.clear();
            count = 0;
            unsafe {
                println!(
                    "attempts: {} ::: max: {} ::: time: {}",
                    C.to_string().blue().italic(),
                    &ANSWER[0..MAX as usize].green().bold(),
                    time.format("%d/%m/%Y %H:%M:%S")
                        .to_string()
                        .yellow()
                        .italic()
                        .bold(),
                );
                if C == u128::MAX - 1 {
                    OVERFLOW_TIMES += 1;
                    C = 0;
                }
                C += 1
            }
        }
    }
}

fn gen_char() -> char {
    CHARS
        .chars()
        .nth(thread_rng().gen_range(0..CHARS.len()))
        .unwrap()
}

fn check_char(ch: char, c: usize) -> bool {
    ANSWER.chars().nth(c).unwrap() == ch
}

use std::io::Write;

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
            "[Times: {:>2}] ::: [Attempt at this time: {:<39}] ::: [Current text: {:<temp$}] ::: [Current time: {}]",
            OVERFLOW_TIMES, C, &ANSWER[0..MAX as usize], chrono::offset::Local::now().format("%d/%m/%Y %H:%M:%S")
            .to_string()
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
