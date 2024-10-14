pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..=n).sum(); //   vsum().pow(2);
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    let s = (1..=n).map(|x| (1..=n).map(move |y| (x as i32, y as i32))).flatten().collect::<Vec<(i32, i32)>>();
    let diff: i32 = s.iter().map(|(x, y)| (x - y).pow(2)).sum();
    diff as u32 / 2
}
