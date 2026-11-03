pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(x: i64) -> i64 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_works() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
