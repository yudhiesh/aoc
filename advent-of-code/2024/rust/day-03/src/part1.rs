use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

#[tracing::instrument(ret)]
pub fn get_digits(text: &str) -> Vec<u32> {
    RE.captures_iter(text)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .collect()
}

#[tracing::instrument(ret)]
pub fn process(input: &str) -> miette::Result<u32> {
    let total: u32 = get_digits(input).into_iter().sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(input)?);
        Ok(())
    }
}
