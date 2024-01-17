trait TwoPairs<T: Clone + Sized>: Clone + Sized {
    fn two_pairs(self) -> (
        (T, T),
        (T, T),
    ) {
        let transformed = self.transform();
        let pair = two_copies(&transformed);
        (pair.clone(), pair)
    }

    fn transform(self) -> T;
}


fn two_copies<T: Clone>(x: &T) -> (T, T) {
    (x.clone(), x.clone())
}

impl TwoPairs<&'static str> for String {
    fn transform(self) -> &'static str {
        "placeholder"
    }
}

impl TwoPairs<String> for String {
    fn transform(self) -> String {
        self
    }
}

#[derive(Debug, Clone)]
struct Demo;

fn main() {
    let hello = "hello".to_string();
    let (s1, mut s2) = two_copies(&hello);
    let _ = s2.pop();
    println!("{} {}", s1, s2);

    let (d1, d2) = two_copies(&Demo);
    println!("{:?} {:?}", d1, d2);

    let p: ((&str, _), (_, _)) = hello.clone().two_pairs();
    println!("{:?}", p);

    let p: ((String, _), (_, _)) = hello.two_pairs();
    println!("{:?}", p);
}
