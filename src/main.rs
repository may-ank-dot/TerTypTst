use rand::Rng;
use std::time::Instant;
use std::io::{self,Write};
use std::fs;

fn main() {
    // let sentences = vec![
    //     "rust makes systems programming fun",
    //     "ownership prevents memory bugs",
    //     "fearless concurrency is powerful",
    //     "typing fast is a useful skill",
    // ];
    let f = fs::read_to_string("Sentence.txt")
        .expect("unable to read the file!"); 
    let sentences:Vec<String> = f
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let sentence = &sentences[rand::thread_rng().gen_range(0..sentences.len())];
    if sentence.len() == 0{
        panic!("Something went wrong!");
    } 
    println!("Type this sentence:\n\n{}\n",sentence);
    println!("Press enter when ready");

    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    print!("Start typing: ");
    io::stdout().flush().unwrap();

    let start = Instant::now();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let duration = start.elapsed().as_secs_f64();
    let input = input.trim();

    let correct_chars = sentence
        .chars()
        .zip(input.chars())
        .filter(|(a,b)| a==b)
        .count();

    let accuracy = (correct_chars as f64 / sentence.chars().count() as f64) * 100.0;

    let words = sentence.split_whitespace().count();
    let wpm = (words as f64 / duration) * 60.0;

    println!("\n📊 Results");
    println!("⏱️  Time Taken: {:.2} seconds", duration);
    println!("🚀 Speed: {:.2} WPM", wpm);
    println!("🎯 Accuracy: {:.2}%", accuracy);

    if accuracy > 95.0 {
        println!("🔥 Excellent typing!");
    } else if accuracy > 80.0 {
        println!("👍 Good job!");
    } else {
        println!("💪 Keep practicing!");
    }
}

