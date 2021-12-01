mod data;

pub fn count_increase()->i32 {
    let mut count: i32=0;
    let total = data::INPUT1.len();
    for i in 1..total {
        if data::INPUT1[i]> data::INPUT1[i-1]{
            count+=1;
        }
    }
    count
}

pub fn get_sums()-> std::vec::Vec<i32> {
    let mut sums: std::vec::Vec<i32>= std::vec::Vec::new();
    let total = data::INPUT1.len();
    for i in 2..total {
        sums.push( data::INPUT1[i] + data::INPUT1[i-1] +data::INPUT1[i-2]);
    }
    sums
}

pub fn count_increase_sums()->i32 {
    let mut count: i32=0;
    let sums = get_sums();
    let total = sums.len();
    for i in 1..total {
        if sums[i]> sums[i-1]{
            count+=1;
        }
    }
    count
}
