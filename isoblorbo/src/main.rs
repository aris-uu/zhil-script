use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::zip;

fn main() {
    println!(
        "{}",
        is_isomorphic("anacyclus".to_string(), "anaeretic".to_string())
    )
}

fn is_isomorphic(a: String, b: String) -> bool {
    let mut mapping: HashMap<char, char> = HashMap::new();
    let mut mapped: HashSet<char> = HashSet::new();

    if a.len() != b.len() {
        return false;
    }
    for i in zip(a.chars(), b.chars()) {
        if mapping.contains_key(&i.0) {
            if mapping.get(&i.0).unwrap() != &i.1 {
                return false;
            }
        } else {
            if mapped.contains(&i.1) {
                return false;
            }
            mapping.insert(i.0, i.1);
            mapped.insert(i.1);
        }
    }
    true
}
