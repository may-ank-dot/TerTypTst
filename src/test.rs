use std::io::{self, Write};
use std::time::Instant;

pub struct TestResult {
    pub input: String,
    pub duration: f64,
}

pub fn run_typing_test(sentence: &str) -> TestResult {
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    print!("Start typing: ");
    io::stdout().flush().unwrap();

    let start = Instant::now();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let duration = start.elapsed().as_secs_f64();

    TestResult {
        input: input.trim().to_string(),
        duration,
    }
}

