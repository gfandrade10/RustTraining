//Rank names

fn rank<'x>(st: &'x str, we: &Vec<i32>, n: usize) -> &'x str 
{
    if st.is_empty() { return "No participants"; }

    use std::collections::HashMap;
    let letters = HashMap::<char, i32>::from([
        ('A', 1), ('B', 2), ('C', 3), ('D', 4), ('E', 5),
        ('F', 6), ('G', 7), ('H', 8), ('I', 9), ('J', 10),
        ('K', 11), ('L', 12), ('M', 13), ('N', 14), ('O', 15),
        ('P', 16), ('Q', 17), ('R', 18), ('S', 19), ('T', 20),
        ('U', 21), ('V', 22), ('W', 23), ('X', 24), ('Y', 25),
        ('Z', 26)]);

    fn score(s: &str, hm: &HashMap<char, i32>) -> i32 {
        let upper_str = s.to_uppercase();
        let mut sum: i32 = 0;
        for x in upper_str.chars() {
            if x >= 'A' && x <= 'Z' {
                sum += hm.get(&x).unwrap();
            }
            else { return 0; }
        }
        sum + (s.len() as i32)
    }

    struct Elements<'x>{
        name: &'x str,
        score: i32, 
    }

    let mut elements: Vec<Elements> = st.split(",").zip(we.iter())
        .map(|(s, &w)| {
            let temp = Elements {
                name: s,
                score: score(s, &letters) * w,
            };
            temp
        }).collect();

    if n > elements.len() { return "Not enough participants"; }

    elements.sort_by(|e1, e2| e2.score.cmp(&e1.score).then(e1.name.cmp(e2.name)));
    for x in &elements { println!("{}, {}", x.name, x.score);}
    elements.iter().nth(n - 1).unwrap().name
}

fn main() 
{
    let st = String::from("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin");
    let vec: Vec<i32> = vec![4, 2, 1, 4, 3, 1, 2];
    let n: usize = 4;
    let res: &str = rank(&st, &vec, n);
    println!("\n{res}\n");
}
