use std::collections::HashMap;

fn main() {
    let mut scores : HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert("Umesh".to_string(), 12);
    scores.insert(String::from("Amrita"), 14);

    print_hash_map(&scores);

    let umesh_value = scores.get(&String::from("Umesh"));
    println!("Umesh's score : {:?}", umesh_value);

    scores.remove(&String::from("Alice"));
    print_hash_map(&scores);

}

fn print_hash_map(scores : &HashMap<String, i32>){
    
    for (key, value) in scores {
        println!("{} -> {}", key, value);
    }
}
