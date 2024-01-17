use std::sync::{OnceLock, RwLock};

struct Input<'a> {
    resource: u8,
    access_counter: &'a RwLock<AccessCounter>,
}

impl<'a> Input<'a> {
    fn new(resource: u8) -> Self {
        let access_counter = ACCESS_COUNTER.get().unwrap();
        Self {
            resource,
            access_counter,
        }
    }

    fn consume(&mut self) -> Option<u8> {
        self.access_counter.write().unwrap().access();
        if self.resource < 1 {
            None
        } else {
            let result = self.resource;
            self.resource -= 1;
            Some(result)
        }
    }
}

#[test]
fn test_input() {
    let mut input = Input::new(3);
    let r3 = input.consume().unwrap();
    let r2 = input.consume().unwrap();
    let r1 = input.consume().unwrap();
    assert!(input.consume().is_none());
    assert_eq!([3, 2, 1], [r3, r2, r1]);
    assert_eq!(4, input.accesses());
}

struct Output<'a> {
    access_counter: &'a RwLock<AccessCounter>,
}

impl<'a> Output<'a> {
    fn new() -> Self {
        let access_counter = ACCESS_COUNTER.get().unwrap();
        Self { access_counter }
    }

    fn produce(&mut self, resource: u8) {
        println!("{}", resource);
        self.access_counter.write().unwrap().access();
    }
}

#[derive(Default, Debug)]
struct AccessCounter(usize);

impl AccessCounter {
    fn access(&mut self) {
        self.0 += 1;
    }

    fn accesses(&self) -> usize {
        self.0
    }
}

static ACCESS_COUNTER: OnceLock<RwLock<AccessCounter>> = OnceLock::new();


fn main() {
    ACCESS_COUNTER.set(RwLock::new(AccessCounter::default())).unwrap();
    let mut input = Input::new(2);
    let mut output = Output::new();
    while let Some(r) = input.consume() {
        output.produce(r);
    }
    println!("{}", ACCESS_COUNTER.get().unwrap().read().unwrap().accesses());
}
