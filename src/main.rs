mod sentence;
mod test;
mod stats;

use sentence::get_random_sentence;
use test::run_typing_test;
use stats::print_results;

fn main() {
    let sentence = get_random_sentence("Sentence.txt");

    println!("Type this sentence:\n\n{}\n", sentence);
    println!("Press enter when ready");

    let result = run_typing_test(&sentence);

    print_results(&sentence, &result);
}

