
pub fn first_non_repeating(str: &str) -> Option<char> 
{
    let mut letters = Vec::<(char,i32)>::new();

    str.chars()
    .for_each(|c| 
        {
            let elem = letters.iter_mut().find(|(e,_)| {
                (*e).to_ascii_lowercase() == c.to_ascii_lowercase() });
            if let Some((_, count)) = elem {
                *count += 1;
            } else { letters.push((c,1)); }
        });

    for (key, val) in letters {
        if val == 1 { return Some(key); }
    }
    None
}

fn main() 
{
    let s = "stress";
    let c = first_non_repeating(s);
    println!("{}\n", c.unwrap());
}
