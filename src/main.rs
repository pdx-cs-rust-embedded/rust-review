use prompted::input;

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    fn double(&self) -> Point {
        Point { x: self.x * 2, y: self.y * 2 }
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "pure red"),
            Color::Green => write!(f, "pure green"),
            Color::Blue => write!(f, "pure blue"),
            Color::Rgb(r, g, b) => write!(
                f,
                "mixed color #{:02x}{:02x}{:02x}",
                r,
                g,
                b,
            ),
        }
    }
}

/// Add "The Great" to the end of the name.
fn greaten(name: String) -> String {
    name + " The Great"
}

/// Add "the small" to the end of the name.
fn smallen(name: String) -> String {
    name + " the small"
}

fn main() {
    let base_name = input!("Who are you?: ");
    let name = if base_name.chars().next().unwrap().is_uppercase() {
        greaten(base_name.clone())
    } else {
        smallen(base_name.clone())
    };
    let greetings = ["Helooo…", "Howdy", "Hi"];
    for g in greetings {
        println!("{}", g);
    }
    println!("Hello {}… {}!", base_name, name);

    /*
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    usize, isize
    */

    /*
    memory
      global RAM
      global ROM ("flash")
      stack
      heap
    register
    */

    let x = 1_000_u128;
    let y = 1_000;
    let z = x + y;
    println!("{} {} {}", x, y, z);

    let p = Point::new(1, 2).double();
    println!("{} {}", p.x, p.y);

    let c = Color::Rgb(0xff, 0, 0);
    println!("{}", c);
}
