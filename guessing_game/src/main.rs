use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    loop{
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        
        loop{
            let mut guess = String::new();
            println!("Guess a number [1 - 100]!");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read the guess.");
                
            let guess: u32 = guess.trim().parse().expect("Please enter a number in the range [1 - 100]!");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }
    
        println!("Play again? (y/n)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read the response.");
        let response: char = response.trim().parse().expect("Please enter (y) to replay, or (n) to exit!");
        match response {
            'n' => break,
            _ => println!("Excellent! Let me think of a new number!")
        }
    }
}
