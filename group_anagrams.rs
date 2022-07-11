use std::collections::HashMap;
use std::iter::FromIterator;

fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    // let mut result: Vec<Vec<String>> = Vec::new();
    let mut hash_table: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        let mut chars:Vec<char> = word.chars().collect();
        chars.sort();
        let key = String::from_iter(chars);
        if hash_table.contains_key(&key) {
            let curr = hash_table.get_mut(&key).unwrap();
            curr.push(word)

        } else {
            let mut aux: Vec<String> = Vec::new();
            aux.push(word);
            hash_table.insert(key, aux);
        }
    }
    /* for (_, value) in hash_table.iter() {
        result.push(value.to_vec());
    }*/
    let result: Vec<Vec<String>> = hash_table.values().cloned().collect();
    result
}

fn main() {
    let testcase = ["eat","tea","tan","eat","ate","nat","bat"];
    let testcase = testcase.iter().map(|s| String::from(*s)).collect::<Vec<_>>();

    println!("{:?}", group_anagrams(testcase));
    println!("{:?}", "".to_string());
    println!("{:?}","a".to_string());
}
