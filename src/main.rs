use std::collections::HashMap;
fn main() {
    find_count("got love like a river flowing from east to west".to_string());
    longest_word("got love like a river flowing from east to west".to_string());
}
fn longest_word(word: String) -> String {
    let mut max_val = 0;
    let mut max_key = "";
    let mut contacts = HashMap::new();
    for keep in word.split(" ") {
        let value = keep.chars().count();
        contacts.insert(keep.to_string(), value);
    }
    for (keep, value) in contacts.iter() {
        if *value > max_val {
            max_key = keep;
            max_val = *value;
        }
    }
    println!("{}", max_key.to_string());
    max_key.to_string()
}

fn find_count(word: String) -> usize {
    let mut vec: Vec<String> = Vec::new();

    for i in word.split(" ") {
        vec.push(i.to_string());
    }

    let find: String = vec[vec.len() - 1].clone();
    let count = find.chars().count();
    println!("{}", count);
    count
}
