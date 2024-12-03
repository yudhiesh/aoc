use super::part1::*;

pub fn combinations_without_replacement(levels: &[u32]) -> miette::Result<Vec<Vec<u32>>> {
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
    Ok(_input
        .lines()
        .map(|line| {
            let mut numbers = Vec::with_capacity(5);
            for num in line.split_whitespace() {
                numbers.push(num.parse::<u32>().unwrap());
            }
            safe_level(&numbers)
        })
        .filter(|result| result.as_ref().map_or(false, |&x| x))
        .count()
        .to_string())
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
