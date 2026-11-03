fn main() {
    for i in 1..100 {
        println!("{}: {}", i, fizzbuzz(i)); 
    }
}

fn fizzbuzz(i: i32) -> String {
    let mut s = "".to_string();
    s += if i % 3 == 0 { "Fizz" } else { "" };
    s += if i % 5 == 0 { "Buzz" } else { "" };
    
    return s;
}
