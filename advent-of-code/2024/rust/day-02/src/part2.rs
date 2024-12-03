use super::part1::*;

pub fn safe_level(levels: &Vec<u32>) -> miette::Result<bool> {
    let is_monotonic = check_monotonicity(levels)?;
    let is_difference_in_range = check_adjacent_levels_difference(levels)?;
    let is_safe = is_monotonic && is_difference_in_range;
    Ok(is_safe)
}

pub fn combinations_without_replacement(levels: &Vec<u32>) -> miette::Result<Vec<Vec<u32>>> {
    Ok((0..levels.len())
        .map(|i| {
            levels
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &x)| x)
                .collect()
        })
        .collect())
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut safe_reports: u32 = 0;
    let parsed_levels: Vec<Vec<u32>> = _input
        .lines()
        .map(|levels| {
            levels
                .split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    for levels in parsed_levels.iter() {
        let is_safe = safe_level(levels)?;
        if is_safe {
            safe_reports += 1
        } else {
            let combinations = combinations_without_replacement(levels)?;
            if combinations
                .iter()
                .any(|combination| safe_level(combination).unwrap_or(false))
            {
                safe_reports += 1
            }
        }
    }
    Ok(safe_reports.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
