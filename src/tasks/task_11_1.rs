use crate::utils::file_reader;
use std::collections::HashMap;

enum CacheValue {
    Parts(String, String),
    Transformed(String),
}

fn make_blinks(stones: Vec<String>, blinks_amount: usize) -> usize{
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    let mut result_amount = 0;

    for i in 0..stones.len() {
        
        result_amount += make_blink_for_one(&stones[i],blinks_amount, &mut cache);
    }

    result_amount
}

fn make_blink_for_one(
    stone: &str,
    blinks_amount: usize,
    cache: &mut HashMap<(String, usize), usize>, // Изменение типа ключа
) -> usize {
    if blinks_amount == 0 {
        return 1;
    }

    if let Some(&cached_count) = cache.get(&(stone.to_string(), blinks_amount)) {
        return cached_count;
    }

    if stone == "0" {
        let count = make_blink_for_one("1", blinks_amount - 1, cache);
        cache.insert((stone.to_string(), blinks_amount), count); 
        return count;
    }
    if stone.len() % 2 == 0 {
        let mid = stone.len() / 2;
        let part_first = stone[0..mid].to_string();
        let part_last = stone[mid..].trim_start_matches('0').to_string();

        let part_last = if part_last.is_empty() {
            "0".to_string()
        } else {
            part_last
        };

        let count = make_blink_for_one(&part_first, blinks_amount - 1, cache)
            + make_blink_for_one(&part_last, blinks_amount - 1, cache);

        cache.insert((stone.to_string(), blinks_amount), count); // Ключ обновлен
        return count;
    }

    if let Ok(num) = stone.parse::<u64>() {
        let transformed = (num * 2024).to_string();
        let count = make_blink_for_one(&transformed, blinks_amount - 1, cache);
        cache.insert((stone.to_string(), blinks_amount), count); // Ключ обновлен
        return count;
    }

    1
}

#[allow(dead_code)]
pub fn run() {
    match file_reader::read_file_to_string("./src/data/11.txt") {
        Ok(data) => {
            let stones: Vec<String> = data.trim().split_whitespace().map(|s| s.to_string()).collect();


            let result_25 = make_blinks(stones.clone(), 25);
            println!("11_1 amount of stones: {}", result_25);

            let result_75 = make_blinks(stones.clone(), 75);
            println!("11_2 amount of stones: {}", result_75);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
