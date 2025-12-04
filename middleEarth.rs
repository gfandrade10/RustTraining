use std::collections::{HashMap};

fn good_vs_evil(good: &str, evil: &str) -> String 
{
    let good_map: HashMap<i32,i32> = HashMap::<i32,i32>::from([
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 3),
        (5, 4),
        (6, 10),
    ]);

    let evil_map: HashMap<i32,i32> = HashMap::<i32,i32>::from([
        (1, 1),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 3),
        (6, 5),
        (7, 10),
    ]);

    let good_score = if let Some(it) = good.split(" ").into() {
        let mut element: i32 = 1;
        let mut sum: i32 = 0;
        
        for x in it.into_iter() {
            sum += good_map.get(&element).unwrap() * x.parse::<i32>().unwrap();
            element += 1;
        }
        sum
    } else { 0 };

    let evil_score = if let Some(it) = evil.split(" ").into() {
        let mut element: i32 = 1;
        let mut sum: i32 = 0;
        
        for x in it.into_iter() {
            sum += evil_map.get(&element).unwrap() * x.parse::<i32>().unwrap();
            element += 1;
        }
        sum
    } else { 0 };

    if good_score > evil_score {
        return "Battle Result: Good triumphs over Evil".to_string();
    } else if evil_score > good_score {
        return "Battle Result: Evil eradicates all trace of Good".to_string();
    } else {
        return "Battle Result: No victor on this battle field".to_string();
    }
}

fn main() 
{
    let good_str = "1 0 1 0 0 0".to_string();
    let evil_str = "1 0 0 0 0 0 0".to_string();
    println!("{}\n", good_vs_evil(&good_str, &evil_str));
}
