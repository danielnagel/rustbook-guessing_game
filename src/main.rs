use std::io;
use rand::Rng;

fn main() {
    println!("Rate die Zahl!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Die geheime Zahl ist: {secret_number}");

    println!("Bitte gib deine Schätzung ein.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Zeile");

    println!("Du hast geschätzt: {guess}");
}
