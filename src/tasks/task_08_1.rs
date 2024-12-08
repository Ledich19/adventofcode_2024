use crate::utils::file_reader;

fn set_antinode(
    antinode_arr: &mut Vec<Vec<char>>,
    antennas_array: &Vec<Vec<char>>,
    antenna: (usize, usize),
) {
    for x in 0..antennas_array.len() {
        for y in 0..antennas_array[x].len() {
            let (ax, ay) = antenna;
            if ax == x && ay == y {
                continue;
            }

            if antennas_array[ax][ay] != antennas_array[x][y] {
                continue;
            }

            let shift_x = (ax as isize) - (x as isize);
            let shift_y = (ay as isize) - (y as isize);

            let new_ax = (ax as isize + shift_x) as usize;
            let new_ay = (ay as isize + shift_y) as usize;
            let new_x = (x as isize - shift_x) as usize;
            let new_y = (y as isize - shift_y) as usize;

            if new_ax < antinode_arr.len() && new_ay < antinode_arr[new_ax].len() {
                antinode_arr[new_ax][new_ay] = '#';
            }

            if new_x < antinode_arr.len() && new_y < antinode_arr[new_x].len() {
                antinode_arr[new_x][new_y] = '#';
            }
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    match file_reader::read_file_to_array_of_arrays_by_char("./src/data/08.txt") {
        Ok(data) => {
            let mut antinode_arr = data.clone();

            for (x, row) in data.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    if *cell == '.' || *cell == '#' {
                        continue;
                    }
                    let antenna = (x, y);
                    set_antinode(&mut antinode_arr, &data, antenna);
                }
            }

            let mut count_result = 0;
            for row in antinode_arr.iter() {
                count_result += row.iter().filter(|&&cell| cell == '#').count();
            }

            println!("08_1 results: {:?}", count_result);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
