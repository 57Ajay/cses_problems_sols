mod number_spiral;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = number_spiral::main();
        assert_eq!(result, [8, 1, 15]);
    }
}
