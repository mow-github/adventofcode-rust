mod data_day1;
use data_day1::*;

fn main() {
    part1();
    // part2();
}
//

fn is_basement(floor: i32) -> bool {
    if floor == -1 {
        return true;
    }
    return false;
}

fn part1() {
    let mut floor: i32 = 0;

    for current_char in get_data_string().chars() {
        println!("{current_char}");

        if current_char == '(' {
            println!("## Scanning: ( -character ");
            floor = floor + 1;
        } else if current_char == ')' {
            println!("-> Scanning: ) -character ");
            floor = floor - 1;
        }
    }
    println!("The current floor is: {floor}");
}

fn part2() {
    let mut floor: i32 = 0;
    let mut floor_index: i32 = 0;

    for current_char in get_data_string().chars() {
        println!("{current_char}");

        if current_char == '(' {
            println!("## Scanning: ( -character ");
            floor = floor + 1;
            floor_index = floor_index + 1;
            //
            if is_basement(floor) {
                break;
            }
            //
        } else if current_char == ')' {
            println!("-> Scanning: ) -character ");
            floor = floor - 1;
            floor_index = floor_index + 1;
            //
            if is_basement(floor) {
                break;
            }
            //
        }
    }
    println!("The current floor is: {floor}");
    println!("The current floor_index is: {floor_index}");
}
