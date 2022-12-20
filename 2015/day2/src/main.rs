use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // get the .txt-file manually from the source
    //
    // part1();
    part2();
}

fn part2() {
    let filename = "./src/data_day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: u32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_owned();
        println!("{}. {}", index + 1, line);
        //
        let c: Vec<&str> = line.split("x").collect();
        let mut tmp_arr: [u32; 3] = [0; 3];
        // l w  h
        //
        let l = c[0].parse::<u32>().unwrap();
        let w = c[1].parse::<u32>().unwrap();
        let h = c[2].parse::<u32>().unwrap();
        println!("{} {} {}", l, w, h);
        tmp_arr[0] = l;
        tmp_arr[1] = w;
        tmp_arr[2] = h;
        tmp_arr.sort();

        let shortest_distance = tmp_arr[0] + tmp_arr[0] + tmp_arr[1] + tmp_arr[1];
        let cubic_feet = tmp_arr[0] * tmp_arr[1] * tmp_arr[2];

        total = total + (shortest_distance + cubic_feet);
        println!("shortestdistance: {}", shortest_distance);
        println!("cubic_feet: {}", cubic_feet);
        println!("Total: {}", total);
        println!("");
        println!("");
    }
    //
    println!("{}", total);
}

#[allow(dead_code)]
fn part1() {
    // let str = fs::read_to_string("./src/data_day2.txt").expect("Error in reading the file");

    let filename = "./src/data_day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: u32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_owned();
        println!("{}. {}", index + 1, line);
        //
        let c: Vec<&str> = line.split("x").collect();
        // l w  h
        //
        let factor: u32 = 2;
        let l = c[0].parse::<u32>().unwrap();
        let w = c[1].parse::<u32>().unwrap();
        let h = c[2].parse::<u32>().unwrap();
        println!("{} {} {}", l, w, h);
        //
        let part_a_raw = l * w;
        let part_b_raw = w * h;
        let part_c_raw = h * l;
        //
        let part_a = factor * part_a_raw;
        let part_b = factor * part_b_raw;
        let part_c = factor * part_c_raw;
        //

        let smallest_area = get_smallest_side_area(part_a_raw, part_b_raw, part_c_raw);
        //
        total = total + (part_a + part_b + part_c + smallest_area);
        //
        println!("{}", total);
    }
}

#[allow(dead_code)]
fn get_smallest_side_area(part_a: u32, part_b: u32, part_c: u32) -> u32 {
    let mut smallest = part_a;
    if part_b < part_a {
        smallest = part_b;
    }
    if part_c < smallest {
        smallest = part_c;
    }
    smallest
}
// l w  h
// 2x3x4

// 2*l*w + 2*w*h + 2*h*l

// 2*6 + 2*12 + 2*8 = 52

// 2x3 = 6  (l*w)
// 3x4 = 12 (w*h)
// 4x2 = 8  (h*l)

// areaSmallestSide
