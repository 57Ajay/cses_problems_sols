mod trailing_zeros;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = trailing_zeros::main();
        assert_eq!(result, ());
    }
}
