mod data;
use std::i32;

pub fn get_horizontal_and_vertical()->(i32,i32){
    let mut horizontal:i32 = 0;
    let  mut vertical:i32 = 0;
    let lines:Vec<&str> = data::INPUT1.lines().collect();
    for line in lines {
        let parts:Vec<&str> = line.trim().split_whitespace().collect();
        let command:&str = parts[0];
        let value:i32 = parts[1].parse::<i32>().unwrap();
        match command {
            "down"=> vertical+=value,
            "up"=>vertical-=value,
            "forward"=>horizontal+=value,
            _=>println!("unknown: {}", command)
        }
        println!("{}",parts[0]);
    }
    (horizontal,vertical)
}

pub fn get_horizontal_and_vertical_and_aim()->(i32,i32,i32){
    let mut horizontal:i32 = 0;
    let  mut vertical:i32 = 0;
    let mut aim=0;
    let lines:Vec<&str> = data::INPUT1.lines().collect();
    for line in lines {
        let parts:Vec<&str> = line.trim().split_whitespace().collect();
        let command:&str = parts[0];
        let value:i32 = parts[1].parse::<i32>().unwrap();
        match command {
            "down"=> aim+=value,
            "up"=>aim-=value,
            "forward"=>{
                horizontal+=value;
                vertical+=aim*value;
            },
            _=>println!("unknown: {}", command)
        }
        println!("{}",parts[0]);
    }
    (horizontal,vertical, aim)
}
