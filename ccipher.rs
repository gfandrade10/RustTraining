// caaesa cipher kata in rust

struct CaesarCipher 
{
    shift: u32,
}

impl CaesarCipher 
{
    fn new(shift: u32) -> CaesarCipher 
    {
        CaesarCipher { 
            shift: shift, 
        }
    }

    fn shift_letter(&self, letter: char) -> char
    {
        let local_shift: u8 = (self.shift % 26) as u8;
        let mut val:u8 = letter as u8 + local_shift;
        if val > 'Z' as u8 {
            val = val - ('Z' as u8) + ('A' as u8) - 1;
        }
        return val as char;
    }

    fn shift_letter_backwars(&self, letter: char) -> char
    {
        let local_shift: u8 = 26 - (self.shift % 26) as u8;
        let mut val:u8 = letter as u8 + local_shift;
        if val > 'Z' as u8 {
            val = val - ('Z' as u8) + ('A' as u8) - 1;
        }
        return val as char;
    }

    fn encode(&self, message: &str) -> String 
    {
        message.to_uppercase()
            .chars()
            .map(|x| {
                if x >= 'A' && x <= 'Z' {
                    self.shift_letter(x)
                } else { x }
            })
            .collect()
    }

    fn decode(&self, message: &str) -> String 
    {
        if message.is_empty() { return "".to_string(); }

        let dec: String = message.chars()
            .map(|x| {
                let temp: char;
                if x >= 'A' && x <= 'Z' {
                    temp = self.shift_letter_backwars(x);
                    temp.to_ascii_lowercase()
                } else { x }
            })
            .collect();

        let mut copy = dec.chars();
        match copy.next() {
            Some(first) => first.to_ascii_uppercase().to_string() + &dec[first.len_utf8()..],
            None => "".to_string()
        }
    }
}

fn main() 
{
    let ccipher = CaesarCipher::new(5);
    let input = "Codewars";
    let encoded = ccipher.encode(&input);
    let decoded = ccipher.decode(&encoded);

    println!("{}\n{}\n",
        encoded,
        decoded
    )
}
