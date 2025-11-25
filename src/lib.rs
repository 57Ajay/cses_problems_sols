mod creating_strings;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = creating_strings::main();
        assert!(result != ());
    }
}
