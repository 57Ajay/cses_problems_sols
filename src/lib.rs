mod raab_game_1;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result = raab_game_1::main();
        assert!(result != ());
    }
}
