mod grid_coloring_1;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result_ = grid_coloring_1::main();
        assert!(result_ != ());
    }
}
