mod day1;

#[cfg(test)]
mod tests {
    use crate::day1::count_increase;
    use crate::day1::count_increase_sums;
    #[test]
    fn day1_puzzle1() {
        assert_eq!(1713, count_increase());
    }
    #[test]
    fn sliding_window() {
       assert_eq!(1734, count_increase_sums()); 
    }
}
