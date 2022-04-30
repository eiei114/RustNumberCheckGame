use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("数を当ててみて！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("秘密の数字は:{}", secret_number);
    loop {
        println!("予想を入力してみて");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("次のように予想しました:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("完璧！");
                break;
            }
        }
    }
}
