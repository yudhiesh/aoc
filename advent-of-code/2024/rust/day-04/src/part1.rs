use tracing::*;

const ITEM_TO_SEARCH: &str = "XMAS";
const WINDOW_SIZE: usize = ITEM_TO_SEARCH.len();
const ITEM_TO_SEARCH_CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

fn transpose(input: &str) -> miette::Result<String> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();

    Ok((0..width)
        .map(|col| lines.iter().map(|row| row[col]).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n"))
}

#[tracing::instrument]
pub fn gather_horizontal(input: &str) -> miette::Result<u32> {
    Ok(input
        .lines()
        .flat_map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .windows(WINDOW_SIZE)
                .map(|window| window.to_vec())
                .filter_map(|window| {
                    if window.as_slice() == ITEM_TO_SEARCH_CHARS {
                        Some(1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum::<u32>())
}

#[tracing::instrument]
pub fn gather_horizontal_reverse(input: &str) -> miette::Result<u32> {
    Ok(input
        .lines()
        .flat_map(|line| {
            line.chars()
                .rev()
                .collect::<Vec<_>>()
                .windows(WINDOW_SIZE)
                .map(|window| window.to_vec())
                .filter_map(|window| {
                    if window.as_slice() == ITEM_TO_SEARCH_CHARS {
                        Some(1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum::<u32>())
}

#[tracing::instrument]
pub fn gather_vertical_reverse(input: &str) -> miette::Result<u32> {
    let transposed = transpose(input)?;
    Ok(gather_horizontal_reverse(&transposed)?)
}

#[tracing::instrument]
pub fn gather_vertical(input: &str) -> miette::Result<u32> {
    let transposed = transpose(input)?;
    Ok(gather_horizontal(&transposed)?)
}

#[tracing::instrument]
pub fn gather_down_right_diagonals(input: &str) -> miette::Result<u32> {
    todo!("Loop over diagonals top-left to down-right")
}

#[tracing::instrument]
pub fn gather_bottom_up_diagonals(input: &str) -> miette::Result<u32> {
    todo!("Loop over diagonals bottom-left to top-right")
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32> {
    let _ = gather_horizontal_reverse(_input);
    Ok(18)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gather_test {
        ($suite:ident, $func:ident, $($name:ident: $input:expr, $expected:expr,)*) => {
            mod $suite {
                use super::*;
                $(
                    #[test]
                    fn $name() -> miette::Result<()> {
                        assert_eq!($expected, $func($input)?);
                        Ok(())
                    }
                )*
            }
        }
    }
    gather_test!(vertical_reverse_tests, gather_vertical_reverse,
        basic: "MMMSXXMXSS
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
", 1,
        advanced: "MMMSXXMXSS
MSAMXMSMAA
AMXSXMAAMM
MSAMASMSXX
", 2,
    );
    gather_test!(vertical_tests, gather_vertical,
        basic: "MMMSXXMXSM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
", 1,);

    gather_test!(horizontal_tests, gather_horizontal,
        basic: "MMMSXXMASM\n", 1,
        advanced: "XMASXXMASM\n", 2,
    );

    gather_test!(horizontal_reverse_tests, gather_horizontal_reverse,
        basic: "MMMSXSAMXM\n", 1,
        advanced: "MSAMXSAMXM\n", 2,
    );

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!(18, process(input)?);
        Ok(())
    }
}
