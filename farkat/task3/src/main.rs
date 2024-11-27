use std::collections::HashMap;

fn main() {
    let mut vowels:HashMap<String, i32> = HashMap::new();
    vowels.insert("a".parse().unwrap(), 1);
    vowels.insert("e".parse().unwrap(), 1);
    vowels.insert("i".parse().unwrap(), 1);
    vowels.insert("o".parse().unwrap(), 1);
    vowels.insert("u".parse().unwrap(), 1);

    let mut count = 0;
    let input_string = String::from("hello world yyy aiu");
    for char in input_string.chars() {
        if vowels.contains_key(&char.to_string()) {
            count += 1;
        }
    }
    println!("task 3\ninput: {}\noutput: vowels count = {}", input_string, count);
}
