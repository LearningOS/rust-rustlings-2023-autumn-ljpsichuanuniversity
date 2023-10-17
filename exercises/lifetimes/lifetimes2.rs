// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



use std::borrow::Cow;

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> Cow<'a, str>
    where 'b: 'a
{
    if x.len() > y.len() {
        Cow::Borrowed(x)
    } else {
        Cow::Borrowed(y)
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()).to_string();
        println!("The longest string is '{}'", result);
    }
    println!("The longest string is '{}'", result);
}
