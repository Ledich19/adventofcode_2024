use std::collections::HashSet;

use crate::utils::file_reader;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum GuardDirection {
    Up,    // '^'
    Down,  // 'v'
    Left,  // '<'
    Right, // '>'
}

#[derive(Eq, Hash, PartialEq)]
pub enum FieldSymbol {
    Guard,       // '^'
    Block,       // '#'
    Space,       // '.'
    Visit,       // 'X'
    Obstruction, // 'O'
}

impl FieldSymbol {
    pub fn to_char(&self) -> char {
        match self {
            FieldSymbol::Guard => '^',
            FieldSymbol::Block => '#',
            FieldSymbol::Space => '.',
            FieldSymbol::Visit => 'X',
            FieldSymbol::Obstruction => 'O',
        }
    }
}

fn is_block_at_front(
    field: &Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: &GuardDirection,
) -> bool {
    let (x, y) = guard_position;
    let (nx, ny) = match guard_direction {
        GuardDirection::Up => (x - 1, y),
        GuardDirection::Down => (x + 1, y),
        GuardDirection::Left => (x, y - 1),
        GuardDirection::Right => (x, y + 1),
    };

    field[nx as usize][ny as usize] == FieldSymbol::Block.to_char()
        || field[nx as usize][ny as usize] == FieldSymbol::Obstruction.to_char()
}

fn is_border_at_front(
    field: &Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: &GuardDirection,
) -> bool {
    let (x, y) = guard_position;
    let (nx, ny) = match guard_direction {
        GuardDirection::Up => (x - 1, y),
        GuardDirection::Down => (x + 1, y),
        GuardDirection::Left => (x, y - 1),
        GuardDirection::Right => (x, y + 1),
    };

    nx < 0 || ny < 0 || nx >= field.len() as i32 || ny >= field[0].len() as i32
}

fn get_guard_position(field: &Vec<Vec<char>>, char_of_guard: char) -> Option<(i32, i32)> {
    for (i, row) in field.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == char_of_guard {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn move_guard(
    field: &mut Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: &GuardDirection,
) -> (i32, i32) {
    let (x, y) = guard_position;
    let (nx, ny) = match guard_direction {
        GuardDirection::Up => (x - 1, y),
        GuardDirection::Down => (x + 1, y),
        GuardDirection::Left => (x, y - 1),
        GuardDirection::Right => (x, y + 1),
    };

    if nx < 0 || ny < 0 || nx >= field.len() as i32 || ny >= field[0].len() as i32 {
        return guard_position;
    }

    if field[nx as usize][ny as usize] == FieldSymbol::Space.to_char()
        || field[nx as usize][ny as usize] == FieldSymbol::Visit.to_char()
    {
        field[x as usize][y as usize] = FieldSymbol::Visit.to_char();
        field[nx as usize][ny as usize] = FieldSymbol::Guard.to_char();
        (nx, ny)
    } else {
        guard_position
    }
}

fn move_guard_2(
    field: &mut Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: &GuardDirection,
) -> (i32, i32) {
    let (x, y) = guard_position;
    let (nx, ny) = match guard_direction {
        GuardDirection::Up => (x - 1, y),
        GuardDirection::Down => (x + 1, y),
        GuardDirection::Left => (x, y - 1),
        GuardDirection::Right => (x, y + 1),
    };

    if nx < 0 || ny < 0 || nx >= field.len() as i32 || ny >= field[0].len() as i32 {
        return guard_position;
    }

    if field[nx as usize][ny as usize] == FieldSymbol::Space.to_char()
        || field[nx as usize][ny as usize] == FieldSymbol::Visit.to_char()
    {
        field[x as usize][y as usize] = FieldSymbol::Space.to_char();
        field[nx as usize][ny as usize] = FieldSymbol::Guard.to_char();
        (nx, ny)
    } else {
        guard_position
    }
}

fn rotate_guard(guard_direction: &GuardDirection) -> GuardDirection {
    match guard_direction {
        GuardDirection::Up => GuardDirection::Right,
        GuardDirection::Right => GuardDirection::Down,
        GuardDirection::Down => GuardDirection::Left,
        GuardDirection::Left => GuardDirection::Up,
    }
}

fn count_symbols(field: &Vec<Vec<char>>, symbols: &[char]) -> i32 {
    let mut count = 0;
    for row in field.iter() {
        for &cell in row.iter() {
            if symbols.contains(&cell) {
                count += 1;
            }
        }
    }
    count
}

fn find_obstructions(field: &Vec<Vec<char>>, visited_field: &Vec<Vec<char>>) -> i32 {
    let mut valid_obstructions = 0;
    let start_guard_position = get_guard_position(field, FieldSymbol::Guard.to_char());
    let mut my_field = field.clone();
    let mut visited: HashSet<(i32, i32, GuardDirection)> = HashSet::new();

    for (i, row) in field.iter().enumerate() {
        for (j, _cell) in row.iter().enumerate() {
            if visited_field[i][j] != FieldSymbol::Visit.to_char()
                && visited_field[i][j] != FieldSymbol::Guard.to_char()
            {
                continue;
            }

            my_field[i][j] = FieldSymbol::Obstruction.to_char();

            let mut guard_direction = GuardDirection::Up;
            let mut guard_position = get_guard_position(&my_field, FieldSymbol::Guard.to_char());

            while guard_position.is_some() {
                let guard_state = (
                    guard_position.unwrap().0,
                    guard_position.unwrap().1,
                    guard_direction.clone(),
                );
                if is_border_at_front(&my_field, guard_position.unwrap(), &guard_direction) {
                    break;
                }
                if visited.contains(&guard_state) {
                    valid_obstructions += 1;
                    break;
                }

                visited.insert(guard_state.clone());

                if is_block_at_front(&my_field, guard_position.unwrap(), &guard_direction) {
                    guard_direction = rotate_guard(&guard_direction);
                } else {
                    move_guard_2(&mut my_field, guard_position.unwrap(), &guard_direction);
                }
                guard_position = get_guard_position(&my_field, FieldSymbol::Guard.to_char());
            }

            my_field[i][j] = FieldSymbol::Space.to_char();

            for row in my_field.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell == FieldSymbol::Guard.to_char() {
                        *cell = FieldSymbol::Space.to_char();
                    }
                }
            }

            my_field[start_guard_position.unwrap().0 as usize]
                [start_guard_position.unwrap().1 as usize] = FieldSymbol::Guard.to_char();

            visited.clear();
        }
    }

    valid_obstructions
}
#[allow(dead_code)]
pub fn run() {
    let mut amount_1 = 0;
    let mut amount_2 = 0;
    #[allow(unused_assignments)]
    let mut copy_field: Vec<Vec<char>> = Vec::new();

    match file_reader::read_file_to_array_of_arrays_by_char("./src/data/06.txt") {
        Ok(mut field) => {
            copy_field = field.clone();
            let mut guard_direction = GuardDirection::Up;
            let mut guard_position = get_guard_position(&field, FieldSymbol::Guard.to_char());

            while guard_position.is_some() {
                if is_border_at_front(&field, guard_position.unwrap(), &guard_direction) {
                    break;
                }

                if is_block_at_front(&field, guard_position.unwrap(), &guard_direction) {
                    guard_direction = rotate_guard(&guard_direction);
                } else {
                    move_guard(&mut field, guard_position.unwrap(), &guard_direction);
                }
                guard_position = get_guard_position(&field, FieldSymbol::Guard.to_char());
            }

            amount_1 = count_symbols(
                &field,
                &[FieldSymbol::Guard.to_char(), FieldSymbol::Visit.to_char()],
            );

            amount_2 = find_obstructions(&copy_field, &field);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("06_1 Amount: {}", amount_1);
    println!("06_2 Amount: {}", amount_2);
}
