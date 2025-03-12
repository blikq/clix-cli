use clix_cli::entry;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let result = entry();

        assert_eq!(String::from("lambo"), result);
    }
}
