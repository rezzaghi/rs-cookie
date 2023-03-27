#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = set("test", "1234");
        assert_eq!(result, "Set-Cookie: test=1234");
    }
}
