use crate::test::TestResult;

pub fn print_results(sentence: &str, result: &TestResult) {

    if result.input.is_empty() || result.duration < 2.0 {
        println!("Invalid or too short input.");
        return;
    }
    let correct_chars = sentence
        .chars()
        .zip(result.input.chars())
        .filter(|(a, b)| a == b)
        .count();

    let expected = sentence.chars().count();
    let typed = result.input.chars().count();
    let total = expected.max(typed);
    
    let accuracy = (correct_chars as f64 / total as f64) * 100.0;

    let chars_typed = result.input.chars().count();
    let minutes = result.duration / 60.0;
    let wpm = (chars_typed as f64 / 5.0) / minutes;

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

