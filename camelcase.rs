//Camel case Kata

fn camel_case(str: &str) -> String 
{
    if str.is_empty() {
        return "".to_string();
    }

    let words: String = str
        .split_whitespace()
        .map(|word| {
            let alive_copy = word.to_string();
            let mut copy = alive_copy.chars();
            match copy.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + copy.as_str(),
                None => "".to_string(),
            }
        })
        .collect();

    words
}

fn main() 
{
    let s = "camel case method".to_string();
    println!("{}\n", camel_case(&s));
}
