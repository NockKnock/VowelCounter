use std::io;

fn main() {
    loop {
        let user_sentence: String = get_user_sentence();
        println!("You input: {}", user_sentence);
        println!(
            "There are {} vowels in your sentence.",
            vowel_counter(&user_sentence)
        );
    }
}

fn get_user_sentence() -> String {
    let mut user_sentence: String = String::new();

    println!("Please input your sentence:");

    io::stdin()
        .read_line(&mut user_sentence)
        .expect("Could not read sentence");
    return user_sentence;
}

fn vowel_counter(user_sentence: &String) -> i32 {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    user_sentence
        .trim()
        .chars()
        .filter(|&c| vowels.contains(&c))
        .count() as i32
}
