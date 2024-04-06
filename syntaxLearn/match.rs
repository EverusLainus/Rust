fn main(){
    let n = 'x';
    match n {
        '0'..='9' =>println!( " a number {}", n),
        'a' | 'b' |  'c' => println!("key words {}", n),
        key if key.is_lowercase() => println!("lowercase {}", key),
        _ => println!("wildcard "),
    }
}