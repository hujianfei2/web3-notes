use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 *  猜数字游戏
 */
fn main() {
    println!("guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);

   loop{
       println!("please input your guess !");
       let mut guess = String::new();
       io::stdin()
           .read_line(&mut guess)
           .expect("failed read line!");
       let guess:u32 = match guess.trim().parse(){
           Ok(num) => num,
           Err(_) => continue,
       };
       println!("you guess number {}", guess);

       match guess.cmp(&secret_number) {
           Ordering::Less => println!("too small!"),
           Ordering::Greater => println!("too big!"),
           Ordering::Equal => {
               println!("you win!");
               break;
           },
       }
   }
}
