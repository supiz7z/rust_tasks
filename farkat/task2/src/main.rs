fn main() {
    let a = 100_i32;
    println!("task 2\ninput: {}\noutput: {}", a, make_negative(a));
}

fn make_negative(i: i32) -> i32 { if i <= 0 { i } else { -i } }
