use crate::test::TestResult;

pub fn print_results(sentence: &str, result: &TestResult) {
    let correct_chars = sentence
        .chars()
        .zip(result.input.chars())
        .filter(|(a, b)| a == b)
        .count();

    let accuracy =
        (correct_chars as f64 / sentence.chars().count() as f64) * 100.0;
    if accuracy == 0.0 {
        println!("Something went wrong!");
        return 
    }

    let words = sentence.split_whitespace().count();
    let wpm = (words as f64 / result.duration) * 60.0;

    println!("\n📊 Results");
    println!("⏱️  Time Taken: {:.2} seconds", result.duration);
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

