#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<&str> {
    Ok("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
