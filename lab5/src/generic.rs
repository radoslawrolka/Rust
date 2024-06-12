#![allow(dead_code)]
pub fn maxy<T>(array: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if array.is_empty() {
        return None;
    }
    let mut max = array[0];
    for &item in array.iter() {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn add(&self, other: &Pair<T>) -> Pair<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        Pair {
            first: self.first + other.first,
            second: self.second + other.second,
        }
    }
}

pub fn run() {
    let array = [1, 2, 3, 4, 5];
    let arrayf64 = [1.1, 2.2, 3.3, 4.4, 5.5];
    let max = maxy(&array);
    println!("max: {:?}", max);
    let maxf64 = maxy(&arrayf64);
    println!("maxf64: {:?}", maxf64);

    let pair1 = Pair {
        first: 1,
        second: 2,
    };
    let pair2 = Pair {
        first: 3,
        second: 4,
    };
    let pair3 = pair1.add(&pair2);
    println!("pair3.first: {:?}", pair3.first);
    println!("pair3.second: {:?}", pair3.second);

}
