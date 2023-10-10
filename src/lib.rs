pub fn get_file() -> &'static str {
    file!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_file(), "src/lib.rs");
    }
}
