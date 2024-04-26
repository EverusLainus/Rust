//let else expression

fn try_let_else(maybe_string:Option<String>)->Result<u32, String>{
    /*
    option<T> is enum that takes some<T> or none
    * s has s or Err
    */
    let s = if let Some(s) = maybe_string{
        s
    }else{
        return Err(String::from("got none"));
    };

    /*
    * get the first character of the string
    */
    let first_bytes = if let Some(first_bytes) = s.chars().next(){
        first_bytes
    }else{
        return Err(String::from("got an empty string"))
    };

    if let Some(digit) = first_bytes.to_digit(16){
        Ok(digit)
    }else{
        Err(String::from("not a hex decimal"))
    }


}

fn main(){
    println!("ans: {:?}", try_let_else(Some(String::from("foo"))));
}