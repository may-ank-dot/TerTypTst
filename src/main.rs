mod sentence;
mod test;
mod stats;
mod initialize;

fn main() {
    let para_size: i32 = initialize::home();
    let sentence = match para_size {
        1 => sentence::get_random_sentence("Paragraph/short.txt"),
        2 => sentence::get_random_sentence("Paragraph/medium.txt"),
        3 => sentence::get_random_sentence("Paragraph/long.txt"),
        _ => panic!("Something went wrong!"),  
    };

    println!("Type this sentence:\n\n{}\n", sentence);
    println!("Press enter when ready");

    let result = test::run_typing_test(&sentence);

    stats::print_results(&sentence, &result);
}

