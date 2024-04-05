// nothing to import to use String::

fn main(){
    //like slice from a char array
    let a : &str = "hello";
    println!("a is {}", a);

    //heap allocated
    let mut s : String = String::from("World");
    println!("s is {}", s);

    s.push_str(a);
    println!("s is {}", s);

    let k : &str = &s[3..];
    println!("k is {}", k);
}