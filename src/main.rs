use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!猜数");
		let secret_number = rand::thread_rng().gen_range(1..101);
		//println!("目前的神秘数字是 :{}",secret_number);

    loop{
    println!("请输入你想的数字.");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess:u32 = guess.trim().parse().expect("Pleasure type a number");    
    println!("你猜想的数字是: {}", guess);
    
    
    match guess.cmp(&secret_number){
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => 
                        {println!("You win");
                        break;}
      

    }
  }

}
