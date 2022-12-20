fn main() {
    part1();
}

fn part1() {
    let data = String::from_utf8(
        "20x3x11
    15x27x5",
    )
    .unwrap();

    println!("Hello, world!");

    let data_splitted = data.split("\n");

    println!("{:?}", data_splitted);
}

// l w  h
// 2x3x4

// 2*l*w + 2*w*h + 2*h*l

// 2*6 + 2*12 + 2*8 = 52

// 2x3 = 6  (l*w)
// 3x4 = 12 (w*h)
// 4x2 = 8  (h*l)

// areaSmallestSide
