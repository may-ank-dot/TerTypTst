use rand::Rng;
use std::fs;

pub fn get_random_sentence(path: &str) -> String {
    let content = fs::read_to_string(path)
        .expect("Unable to read the file!");

    let sentences: Vec<String> = content
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.is_empty() {
        panic!("No valid sentence found!");
    }

    let idx = rand::thread_rng().gen_range(0..sentences.len());
    sentences[idx].clone()
}
