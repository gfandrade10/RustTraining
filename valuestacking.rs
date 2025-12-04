use std::fmt::Display;

fn sum_consecurives(numbers: &Vec<i32>) -> Vec<i32>
{
    let mut v = Vec::<i32>::new();
    let mut iterator = numbers.iter();

    if let Some(first) = iterator.next() {
        let mut current_value: i32 = *first;
        let mut current_sum: i32 = *first;

        for &x in iterator {
            if x == current_value {
                current_sum += x;
            }
            else {
                v.push(current_sum);
                current_value = x;
                current_sum = x;
            }
        }
        v.push(current_sum);
    }
    v
}

fn print_array<T>(arr: &Vec<T>)
    where T:Display 
{
    for x in arr {
        print!("{} ", x);
    }
    println!("");
}

fn main() 
{
    let v = vec![1,4,4,4,0,4,3,3,1];
    let mod_v = sum_consecurives(&v);
    print_array(&v);
    print_array(&mod_v);
    println!("");
}
