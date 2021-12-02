mod day1;
mod day2;

#[cfg(test)]
mod tests {
    use crate::day1::count_increase;
    use crate::day1::count_increase_sums;
    use crate::day2::get_horizontal_and_vertical;
    use crate::day2::get_horizontal_and_vertical_and_aim;
    #[test]
    fn day1_puzzle1() {
        assert_eq!(1713, count_increase());
    }
    #[test]
    fn sliding_window() {
       assert_eq!(1734, count_increase_sums()); 
    }
    #[test]
    fn calculate_horizontal_vertical(){
        let (h,v) = get_horizontal_and_vertical();
        println!("h{}",h);
        println!("v{}",v);
        assert_eq!(1670340,h*v);
    }
    #[test]
    fn calculate_horizontal_vertical_aim(){
        let (h,v,a) = get_horizontal_and_vertical_and_aim();
        println!("h{}",h);
        println!("v{}",v);
        println!("a{}",a);
        assert_eq!(1954293920,h*v);
    }

}
