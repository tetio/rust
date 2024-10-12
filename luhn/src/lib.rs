/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let aux = code.replace(" ", "");
    if aux.len() < 2 || aux.matches(char::is_numeric).count() != aux.len() {
        return false;
    }
    let sum: u64 = aux
        .as_bytes()
        .iter()
        .rev()
        .map(|x| x - b'0')
        .enumerate()
        .map(|(i, x)| {
            if (i + 1) % 2 == 0 {
                calculate_digit(x)
            } else {
                x as u64
            }
        })
        .sum();
    sum % 10 == 0
}

fn calculate_digit(n: u8) -> u64 {
    let value = n * 2;
    match value > 9 {
        true => value as u64 - 9,
        false => value as u64,
    }
}



pub fn ok_is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    }