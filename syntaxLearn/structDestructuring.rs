struct Foo{
    a:(i32, i32),
    b: i32,
}

fn main(){
    let foo = Foo{ a:(1,3), b: 5};
    match foo{
        Foo(a:{x, 3}, b: 5) =>println!("a.x: {a.0}, a.y:{a.1}, b:{b}"),
        _ =>println!( "some number"),
    }
}