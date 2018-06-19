extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜一个数（1-100）！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("生成的值为：{}", secret_number);
    loop {
        println!("输入你猜的数字！");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("读取失败！");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // .expect("请输入一个数字");
        println!("你猜的值为：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less   => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("赢了");
                break;
            }
        }
    }
}
