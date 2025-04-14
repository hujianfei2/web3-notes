use std::io::stdin;  // 导入包
fn main() {
    let mut msg:String = String::new();
    println!("Please ent msg:");
    stdin().read_line(&mut msg).unwrap();
    println!("msg is : {}", msg);
}
