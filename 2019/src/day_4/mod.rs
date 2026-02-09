use anyhow::Result;

pub fn run() -> Result<()> {
    let start = 246540;
    let end = 787419;

    let valid_passwords = (start..=end)
        .filter(|password| is_valid_password(*password))
        .count();
    println!("Day 4, Part 1: Number of valid passwords is {valid_passwords}");

    let valid_passwords = (start..=end)
        .filter(|password| is_valid_password_advanced(*password))
        .count();
    println!("Day 4, Part 2: Number of valid passwords advanced is {valid_passwords}");

    Ok(())
}

fn is_valid_password(password: usize) -> bool {
    let parts = password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    if parts.len() > 6 {
        return false;
    }

    // Two adjacents digits are the same
    let two_adjacent_nums = parts.windows(2).any(|digits| digits[0] == digits[1]);
    if !two_adjacent_nums {
        return false;
    }

    // No decreasing values
    let only_increasing = parts.windows(2).all(|digits| digits[0] > digits[1]);
    if !only_increasing {
        return false;
    }

    true
}

fn is_valid_password_advanced(password: usize) -> bool {
    let parts = password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    if parts.len() > 6 {
        return false;
    }

    // Two adjacents digits are the same
    let two_adjacent_nums = (0..5).any(|idx| match idx {
        0 => parts[0] == parts[1] && parts[0] != parts[2],
        4 => parts[4] == parts[5] && parts[4] != parts[3],
        n => parts[n] == parts[n + 1] && parts[n] != parts[n - 1] && parts[n] != parts[n + 2],
    });
    if !two_adjacent_nums {
        return false;
    }

    // No decreasing values
    let only_increasing = parts.windows(2).all(|digits| digits[0] <= digits[1]);
    if !only_increasing {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        assert_eq!(is_valid_password_advanced(112233), true);
        assert_eq!(is_valid_password_advanced(123444), false);
        assert_eq!(is_valid_password_advanced(111122), true);
    }
}
