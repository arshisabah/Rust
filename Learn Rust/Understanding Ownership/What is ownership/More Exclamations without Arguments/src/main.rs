fn main() {

    let hello = String::from("Hello");

    let mut hello1 = hello_with_exclamation(hello);

    println!("{} is `{}`", "hello1", hello1);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn hello_with_exclamation(s: String) -> String {
    let mut str = s;
    str.push_str("!");
    str
}
