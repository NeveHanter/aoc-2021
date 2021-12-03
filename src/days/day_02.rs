const DATA: &str = include_str!("../data/02.txt");

const DIRECTION_DOWN: &str = "down";
const DIRECTION_FORWARD: &str = "forward";
const DIRECTION_UP: &str = "up";

#[allow(unused)]
pub fn day_02() {
    let data: Vec<(&str, i32)> = DATA
        .lines()
        .map(|l| l.splitn(2, " "))
        .map(|mut l| {
            (
                l.next()
                    .unwrap(),
                l.next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            )
        })
        .collect();

    println!("02a: {}\n02b: {}", part_1(&data), part_2(&data));
}

fn part_1(data: &[(&str, i32)]) -> i32 {
    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;

    for (direction, amount) in data {
        match direction {
            &DIRECTION_DOWN => depth += amount,
            &DIRECTION_UP => depth -= amount,
            &DIRECTION_FORWARD => horizontal_position += amount,
            &_ => panic!(),
        }
    }

    horizontal_position * depth
}

fn part_2(data: &[(&str, i32)]) -> i32 {
    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for (direction, amount) in data {
        match direction {
            &DIRECTION_DOWN => aim += amount,
            &DIRECTION_UP => aim -= amount,
            &DIRECTION_FORWARD => {
                horizontal_position += amount;
                depth += amount * aim;
            }
            &_ => panic!(),
        };
    }

    horizontal_position * depth
}
