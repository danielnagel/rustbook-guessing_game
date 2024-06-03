use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Rate die Zahl!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Bitte gib deine Schätzung ein.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Zeile");

    let guess: u32 = guess.trim().parse().expect("Bitte gib eine Zahl ein!");

    println!("Du hast geschätzt: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Zu klein!"),
        Ordering::Greater => println!("Zu groß!"),
        Ordering::Equal => println!("Du hast gewonnen!"),
    }
}
