pub fn run() {
    let a = [1,2,3];
    let b = [1,2];
    let c = len_longer_array(&a, &b);
    println!("{:?}", &c);
}

fn len_longer_array<'a>(a : &'a[i32], b : &'a[i32]) -> &'a[i32] {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}