const DATA: &str = include_str!("../data/01.txt");

#[allow(unused)]
pub fn day_01() {
    let numbers: Vec<i32> = DATA
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.parse::<i32>()
                .unwrap()
        })
        .collect();

    let increases = count_increases(&numbers);
    let increases_in_sliding_window = count_increases_in_sliding_window(&numbers, 3);

    println!("01a: {}\n01b: {}", increases, increases_in_sliding_window);
}

fn count_increases(numbers: &Vec<i32>) -> i32 {
    numbers
        .iter()
        .zip(
            numbers
                .iter()
                .skip(1),
        )
        .fold(0, |counter, (a, b)| counter + (a < b) as i32)
}

fn count_increases_in_sliding_window(numbers: &Vec<i32>, window_size: usize) -> i32 {
    let numbers: Vec<i32> = numbers
        .windows(window_size)
        .map(|chunk| {
            chunk
                .iter()
                .sum()
        })
        .collect();

    count_increases(&numbers)
}

#[test]
fn test_count_increases() {
    assert_eq!(count_increases(&Vec::new()), 0);

    assert_eq!(count_increases(&vec![1]), 0);
    assert_eq!(count_increases(&vec![1, 2]), 1);
    assert_eq!(count_increases(&vec![1, 2, 3]), 2);

    assert_eq!(count_increases(&vec![1, 2, 1, 3]), 2);
    assert_eq!(count_increases(&vec![-1, 2, 1, 3]), 2);

    assert_eq!(count_increases(&vec![1, -1]), 0);
}

#[test]
fn test_count_increases_in_sliding_window() {
    assert_eq!(count_increases_in_sliding_window(&Vec::new(), 1), 0);

    assert_eq!(count_increases_in_sliding_window(&vec![1], 3), 0);
    assert_eq!(count_increases_in_sliding_window(&vec![1, 2], 3), 0);
    assert_eq!(count_increases_in_sliding_window(&vec![1, 2, 3], 3), 0);
    assert_eq!(count_increases_in_sliding_window(&vec![1, 1, 1, 1], 3), 0);
    assert_eq!(count_increases_in_sliding_window(&vec![2, 1, 1, 1], 3), 0);

    assert_eq!(count_increases_in_sliding_window(&vec![1, 1, 1, 2], 3), 1);
    assert_eq!(
        count_increases_in_sliding_window(&vec![1, 1, 1, 2, 3], 3),
        2
    );
}
