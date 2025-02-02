use chrono::Local;
use rand::seq::IndexedRandom;

/// greeting users
pub fn greet(name: &str) {
    println!("Hello, {}! Welcome to Levy CLI", name)
}

/// func to do add
pub fn calculate(a:i32, b:i32) {
    println!("The sum of {} and {} is {}.", a, b, a + b);
}

/// func to do mpy
pub fn multiply(a: i32, b:i32) {
    println!("The product of {} * {} = {}", a, b, a * b);
}

/// func to do subtract
pub fn subtract(a:i32, b:i32) {
    println!("The difference between {} and {} is {}", a, b, a - b);
}

/// func to do division
pub fn divide(a:i32, b:i32) -> Result<(), String> {
    if b == 0 {
        return Err("Divide by zero is not allowed".into());
    }
    println!("The division of {} by {} is {}.", a, b, a/b);
    Ok(())
}

/// func to get time
pub fn show_time() {
    let now = Local::now();
    println!("Yo, it's {} 🕒 have u done it?", now.format("%H:%M"));
}

/// func to get date
pub fn show_date() {
    let today = Local::now();
    println!("Bro, today is {} 📅, how's goin on?", today.format("%d %B %Y"));
}

/// func to get random emote
pub fn get_emote() {
    let emotes = ["🔥","🚀","😂","💡","🎉","😎","🥳","🤖","💯","✨","🤣","🤬","😱","🥵","💩","🖕","🤙","🤑","💀","🎃","🥶","🤓","😛","🧐"];
    let mut rng = rand::rng();
    if let Some(emote) = emotes.choose(&mut rng) {
        println!("{}", emote);
    }
}