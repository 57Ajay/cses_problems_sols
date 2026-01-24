mod string_reorder;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result_ = string_reorder::main();
        assert!(result_ != ());
    }
}
