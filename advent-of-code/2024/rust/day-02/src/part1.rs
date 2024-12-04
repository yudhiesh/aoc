#[tracing::instrument(ret)]
pub fn safe_level(levels: &[u32]) -> miette::Result<bool> {
    let is_monotonic = check_monotonicity(levels)?;
    let is_difference_in_range = check_adjacent_levels_difference(levels)?;
    let is_safe = is_monotonic && is_difference_in_range;
    Ok(is_safe)
}

#[tracing::instrument(ret)]
pub fn check_monotonicity(levels: &[u32]) -> miette::Result<bool> {
    Ok(levels.windows(2).all(|pair| pair[0] <= pair[1])
        || levels.windows(2).all(|pair| pair[0] >= pair[1]))
}

#[tracing::instrument(ret)]
pub fn check_adjacent_levels_difference(levels: &[u32]) -> miette::Result<bool> {
    Ok(levels.windows(2).all(|pair| {
        let diff = pair[0].abs_diff(pair[1]);
        (1..4).contains(&diff)
    }))
}

#[tracing::instrument(ret, skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut safe_reports: u32 = 0;
    let parsed_levels: Vec<Vec<u32>> = input
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
