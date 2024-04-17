fn main(){
    let mut name= String::from("beautiful day");
    while let Some(c) =  name.pop(){
        println!("{}",c);
    }
}