fn main() {

    let num = 8;
    let mut result_num = 0;
    let mut sum_string:String = Default::default();

    for i in 1..num+1 {
        result_num += i;
        let substring = if i != num {format!("{} + ", i)} else {format!("{}", i)};
        sum_string.push_str(&substring);
    }

    let output = format!("{} -> {} ({})", num, result_num, sum_string);
    println!("task 1\ninput: {}\noutput: {}", num, output);
}
