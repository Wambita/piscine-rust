use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    let mut tries = 0;

    println!("{}", riddle);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        tries += 1;

        let input_trimmed = input.trim();

        if input_trimmed == correct_answer {
            println!("Number of trials: {}", tries);
            break;
        } else {
            println!("{}", riddle);
        }
    }
}