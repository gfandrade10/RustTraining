use std::string::String;
use std::vec::Vec;

struct Pair
{
    s: String,
    index: u32,
}

fn string_splitter(str: &str) -> Vec<Pair>
{
    let tokens = str.split(" ");
    let mut str_arr: Vec<Pair> = Vec::new();
    for s_iter in tokens
    {
        for c in s_iter.chars()
        {
            if c.is_digit(10)
            {
                let temp_pair = Pair {
                    s: s_iter.to_string(),
                    index: c.to_digit(10).unwrap(),
                };
                str_arr.push(temp_pair);
            }
        }
    }
    str_arr
}

fn order(sentence: &str) -> String 
{
    let mut str_tokens = string_splitter(sentence);
    str_tokens.sort_by_key(|elem| elem.index);
    str_tokens.iter().map(|it| it.s.as_str()).collect::<Vec<&str>>().join(" ")
}

fn main() 
{
    let input_str: String = "is2 Thi1s T4est 3a".to_string();
    let result: String = order(&input_str.as_str());
    println!("Original: {}\nSorted: {}", input_str, result);
}
