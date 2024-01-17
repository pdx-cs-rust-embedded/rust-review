#[derive(Debug)]
enum MmeError<'a, T> {
    EmptySlice,
    DuplicateMin(&'a T),
    DuplicateMax(&'a T),
}
use MmeError::*;

/// Return a unique min and max if these exist.
fn min_max_exclusive<'a, T: Ord>(a: &'a [T]) ->
    Result<(&'a T, &'a T), MmeError<'a, T>>
{
    let mut vs = a.iter();
    let v: &T = vs.next().ok_or(EmptySlice)?;
    let (mut min_v, mut max_v) = (v, v);
    let mut min_v_i = 0;
    let mut max_v_i = 0;

    for (i, v) in a.iter().enumerate() {
        if v < min_v {
            min_v = v;
            min_v_i = i;
        }
        if v > max_v {
            max_v = v;
            max_v_i = i;
        }
    }

    for (i, v) in a.iter().enumerate() {
        if v == min_v && i != min_v_i {
            return Err(DuplicateMin(v));
        }
        if v == max_v && i != max_v_i {
            return Err(DuplicateMax(v));
        }
    }

    Ok((min_v, max_v))
}

fn main() {
    let mm = min_max_exclusive(&[3, 7, 5]).unwrap();
    println!("{} {}", mm.0, mm.1);

    let a: [i32; 0] = [];
    let mm = min_max_exclusive(&a);
    match mm {
        Ok(mm) => println!("{} {}", mm.0, mm.1),
        Err(EmptySlice) => println!("empty slice"),
        Err(e) => panic!("{:?}", e),
    }

    let mm = min_max_exclusive(&[3, 7, 3, 5]);
    match mm {
        Ok(mm) => println!("{} {}", mm.0, mm.1),
        Err(DuplicateMin(v)) => println!("dup min {}", v),
        Err(e) => panic!("{:?}", e),
    }
}
