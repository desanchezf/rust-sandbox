pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // Este funciona
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // Este falla
    #[test]
    fn it_fails() {
        panic!("Make this test fail");
    }
}
