use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

pub fn find_uniq<'a>(arr: &'a [&str]) -> &'a str 
{
    fn get_set(word: &str) -> BTreeSet<char>
    {
        word.chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_ascii_lowercase())
            .collect()
    }

    let mut letters: BTreeMap<BTreeSet<char>, usize> = BTreeMap::new();
    for x in arr 
    {
        match letters.entry(get_set(x))
        {
            Entry::Vacant(e) => { e.insert(1); }
            Entry::Occupied(mut e) => { *e.get_mut() += 1; }
        }
    }

    let single_set = letters
        .iter()
        .find(|(_,v)| **v == 1)
        .map(|(k,_)| k);

    for &x in arr
    {
        if get_set(x) == *single_set.unwrap() { return x; }
    }
    ""
} 

fn main() 
{
    let list: &[&str] = &["abc", "acb", "bac", "foo", "bca", "cab", "cba"];
    let unique = find_uniq(list);

    for x in list
    {
        print!("{} ", *x);
    }
    println!("\nUnique: {}\n", unique);
}
