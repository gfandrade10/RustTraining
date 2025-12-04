//check valid ip kata

fn is_valid_ip(ip: &str) -> bool 
{
    if ip.is_empty() { return false; }

    fn check_without_zeros(input: &str) -> bool {
        let number = input.parse::<i32>().unwrap();
        return (format!("{}", number) == input) as bool;
    }

    let mut b: bool = true;
    if let Some(p) = Some(ip.split(".")) {
        let mut counter: i32 = 0;

        for x in p {
            let num = x.parse::<i32>();
            if num.is_ok() {
                let number = num.unwrap();
                b = b && check_without_zeros(&x) &&
                number >= 0 &&
                number < 256;
            } else { b = false; }

            counter += 1;
        }

        b = b && ((counter == 4) as bool);
    };
    
    return b;
}

fn list_result(input: &str) -> &str
{
    if is_valid_ip(&input) {
        return "Valid";
    } else { return "Invalid"; }
}

fn main() 
{
    let valid = "123.45.67.89";
    let invalid = "123.045.067.089";

    println!("{}: {}\n{}: {}\n\n",
        valid,
        list_result(&valid),
        invalid,
        list_result(&invalid)
    )
}
