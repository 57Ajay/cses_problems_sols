mod apple_division;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = apple_division::main();
        assert!(result != ());
    }
}
