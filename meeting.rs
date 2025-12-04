//meeting kata

fn meeting(s: &str) -> String 
{
    struct FName {
        first: String,
        last: String,
    }

    let mut names: Vec<FName> = 
        s.split(";")
        .map(|x| {
            let mut parts = x.split(":");
            FName {
                first: parts.next().unwrap().to_string().to_uppercase(),
                last: parts.next().unwrap().to_string().to_uppercase(),
            }
        })
        .collect();

    names.sort_by_key(|input| (input.last.clone(), input.first.clone()));

    let mut ret_string = String::new();
    for x in names {
        ret_string.push_str(&format!("({}, {})", x.last, x.first));
    }
    ret_string
}

fn main() 
{
    let s = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill".to_string();
    let names = meeting(s.as_str());
    println!("{names}");
}
