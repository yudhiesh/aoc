use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    Ok(_input
        .lines()
        .flat_map(|line| {
            let (left, _) = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            let right_count = _input
                .lines()
                .filter(|l| l.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap() == left)
                .count();

            Some(left * right_count as i32)
        })
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
