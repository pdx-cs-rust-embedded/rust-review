#[derive(Debug)]
struct S<'a>(&'a mut u8);

impl<'a> S<'a> {
    fn new(v: &'a mut u8) -> S {
        S(v)
    }

    fn min(self, v: S<'a>) -> S {
        if self.0 <= v.0 {
            self
        } else {
            v
        }
    }
}

fn main() {
    let mut zero = 0;
    let mut x = S::new(&mut zero);
    let mut one = 1;
    let x1 = S::new(&mut one);
    x = x.min(x1);
    let y = &mut x;
    *y.0 = 1;
    let z = y;
    println!("{:?}", *z);
    println!("{:?}", x);
}
