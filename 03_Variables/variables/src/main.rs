fn main() {
    // Constants are always need to type value must be annotated
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points is {}", MAX_POINTS);
    // Variables default to immutable
    let guess: &str = "42";
    println!("You guessed: {}", guess);
    // Mutable Variables
    let mut guess_mutable = 42;
    println!("You guessed: {}", guess_mutable);
    guess_mutable = 43;
    println!("You guessed: {}", guess_mutable);
    println!("Hello, world!");
}
