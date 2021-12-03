//! All the code assumes little-endian endianness.

const DATA: &str = include_str!("../data/03.txt");
const DATA_LINE_LENGTH: usize = 12;
const RADIX_BINARY: u32 = 2;

#[allow(unused)]
pub fn day_03() {
    let numbers: Vec<u32> = DATA
        .lines()
        .map(|l| u32::from_str_radix(l.trim(), RADIX_BINARY).unwrap())
        .collect();

    let (power_consumption, life_support_rating) = (part_1(&numbers), part_2(&numbers));

    println!("03a: {}\n03b: {}", power_consumption, life_support_rating);
}

fn count_bits(numbers: &Vec<u32>, bit_index: usize) -> (usize, usize) {
    let bit_rev_index = (DATA_LINE_LENGTH - 1) - bit_index;

    let ones = numbers
        .iter()
        .filter(|number| (*number >> bit_rev_index) & 0b1 == 1)
        .count();

    (numbers.len() - ones, ones)
}

fn reduce_rating<P>(numbers: &Vec<u32>, predicate: P) -> u32
where
    P: Fn(usize, usize) -> bool,
{
    let mut numbers = numbers.clone();

    for bit_index in 0..DATA_LINE_LENGTH as usize {
        let bit_rev_index = (DATA_LINE_LENGTH - 1) - bit_index;

        let (zeros, ones) = count_bits(&numbers, bit_index);
        let predicate_value = predicate(ones, zeros);

        numbers = numbers
            .iter()
            .filter(|number| ((*number >> bit_rev_index) & 0b1) == predicate_value as u32)
            .map(|number| *number)
            .collect();

        let numbers_count = numbers.len();
        if numbers_count == 1 {
            return *numbers
                .first()
                .unwrap();
        } else if numbers_count == 0 {
            panic!("Shouldn't happen, no numbers left.")
        }
    }

    unreachable!();
}

fn part_1(numbers: &Vec<u32>) -> u32 {
    let mut bit_counts: [(usize, usize); DATA_LINE_LENGTH] = [(0, 0); DATA_LINE_LENGTH];
    for bit_index in 0..DATA_LINE_LENGTH {
        bit_counts[bit_index as usize] = count_bits(&numbers, bit_index);
    }

    let (mut gamma, mut epsilon) = (0u32, 0u32);
    for (bit_index, (zeroes, ones)) in bit_counts
        .iter()
        .rev()
        .enumerate()
    {
        if *ones >= *zeroes {
            gamma |= 1 << bit_index;
        } else {
            epsilon |= 1 << bit_index;
        }
    }

    gamma * epsilon
}

fn part_2(numbers: &Vec<u32>) -> u32 {
    let oxygen_generator_rating = reduce_rating(&numbers, |zeros, ones| ones >= zeros);
    let co2_scrubber_rating = reduce_rating(&numbers, |zeros, ones| ones < zeros);

    oxygen_generator_rating * co2_scrubber_rating
}
