use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing Game");
    println!("Enter a guess:")

    //Declare variables
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1..=100);

    //guess is a string and secret_num is a number. to allow for future comparisms, guess ought to be converted to a number(u32).
    loop{
    io::stdin().read_line(&mut guess).expect("Failed to readline!!!")
    let guess: u32 = guess.trim().parse(){
        Ok(num) => num;
        Err(_) => continue;
    }

    //match or compare guess and secretkey
    match guess(&secret_num){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("To Big"),
        Ordering::Equal => {
            println!("Correct Guess!")
            break;
        }
    }    
    }
}
