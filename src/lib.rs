pub mod helper {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

pub mod problems;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = helper::add(2, 2);
        assert_eq!(result, 4);
    }
}
