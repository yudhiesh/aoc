use lazy_static::lazy_static;
use regex::Regex;
use tracing::trace;

lazy_static! {
    static ref MULTIPLICATIONS: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref DONT: Regex = Regex::new(r"don\'t\(\)").unwrap();
    static ref DO: Regex = Regex::new(r"do\(\)").unwrap();
}

pub fn get_multiplications(text: &str) -> Vec<(usize, Vec<u32>)> {
    MULTIPLICATIONS
        .captures_iter(text)
        .map(|cap| {
            let pos = cap.get(0).unwrap().start();
            (
                pos,
                vec![
                    cap[1].parse::<u32>().unwrap(),
                    cap[2].parse::<u32>().unwrap(),
                ],
            )
        })
        .collect()
}

#[tracing::instrument(ret)]
pub fn process(input: &str) -> miette::Result<u32> {
    let multiplications = get_multiplications(input);
    let dos: Vec<_> = DO.find_iter(input).map(|m| m.start()).collect();
    let donts: Vec<_> = DONT.find_iter(input).map(|m| m.start()).collect();

    let mut sum = 0;

    for (pos, digits) in multiplications {
        trace!(pos = ?pos, digits = ?digits);
        let last_do = dos.iter().filter(|&&do_pos| do_pos < pos).max();
        let last_dont = donts.iter().filter(|&&dont_pos| dont_pos < pos).max();
        trace!(last_do = last_do);
        trace!(last_dont = last_dont);

        let enabled = match (last_do, last_dont) {
            (Some(do_pos), Some(dont_pos)) => do_pos > dont_pos,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => true,
        };

        if enabled {
            let product: u32 = digits.iter().product();
            sum += product;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input)?);
        Ok(())
    }
}
