use prompted::input;

fn greaten(name: String) -> String {
    name + " The Great"
}

fn smallen(name: String) -> String {
    name + " the small"
}

fn main() {
    let name = input!("Who are you?: ");
    let name = if name.chars().next().unwrap().is_uppercase() {
        greaten(name)
    } else {
        smallen(name)
    };
    let greetings = ["Helooo…", "Howdy", "Hi"];
    for g in greetings {
        println!("{}", g);
    }
    println!("Hello, {}!", name);
    // loop {}
}
