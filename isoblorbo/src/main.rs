use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::fs::File;
use std::iter::zip;
use itertools::Itertools;

fn main() {
    println!(
        "{}",
        is_isomorphic("anacyclus".to_string(), "anaeretic".to_string())
    );
    let words = lines_from_file("../masterwordlist.txt");
    let ciphertexts = "abcdef gchiejek lmnop".split(' ');

    let isomorphic = words.iter().filter(|x| is_isomorphic(x, x));
    for line in ciphertexts {
        println!("{:?}", line);
    }

    
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
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
