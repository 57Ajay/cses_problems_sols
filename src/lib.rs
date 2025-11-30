mod chessboard_and_queens;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = chessboard_and_queens::main();
        assert!(result != ());
    }
}
