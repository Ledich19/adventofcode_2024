use crate::utils::file_reader;

fn parse_ordering_rule(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<i32> = line
        .split('|')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

fn parse_page(line: &str) -> Vec<i32> {
    line.split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}

fn check_pages(pages: &[i32], rules: &[(i32, i32)]) -> bool {
    for &(page_first, page_second) in rules {
        let index_first = pages.iter().position(|&x| x == page_first);
        let index_second = pages.iter().position(|&x| x == page_second);

        if let (Some(index_first), Some(index_second)) = (index_first, index_second) {
            if index_first > index_second {
                return false;
            }
        }
    }
    true
}

fn sort_pages_by_rules(mut pages: Vec<i32>, rules: &[(i32, i32)]) -> Vec<i32> {
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for &(page_first, page_second) in rules {
            let index_first = pages.iter().position(|&x| x == page_first);
            let index_second = pages.iter().position(|&x| x == page_second);

            if let (Some(index_first), Some(index_second)) = (index_first, index_second) {
                if index_first > index_second {
                    pages.swap(index_first, index_second);
                    sorted = false;
                }
            }
        }
    }

    pages
}
#[allow(dead_code)]
pub fn run() {
    let mut amount_1 = 0;
    let mut amount_2 = 0;
    let mut ordering_rules: Vec<(i32, i32)> = Vec::new();
    let mut booklet: Vec<Vec<i32>> = Vec::new();

    match file_reader::read_file_lines("./src/data/05.txt") {
        Ok(lines) => {
            for line in lines {
                if line.contains('|') {
                    if let Some(rule) = parse_ordering_rule(&line) {
                        ordering_rules.push(rule);
                    }
                } else if !line.trim().is_empty() {
                    let pages = parse_page(&line);
                    booklet.push(pages);
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    for pages in &booklet {
        if check_pages(pages, &ordering_rules) {
            amount_1 += pages[(pages.len() - 1) / 2];
        } else {
            let result = sort_pages_by_rules(pages.clone(), &ordering_rules);
            amount_2 += result[(result.len() - 1) / 2];
        }
    }

    println!("05_1 Amount: {}", amount_1);
    println!("05_2 Amount: {}", amount_2);
}
