use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    

    // using rand::thread_rng() to retrieve a thread-based number 
    //generator and use it to generate a number in the range of 1-100 
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Creating a new String variable 
    

    loop {
        println!("Please input your guess. Type 'quit' to exit.");
        // Read from the standard input and assign it to the previously defined variable `guess`.
        // If the io fails for whatever reason print -> "Failed to read line."  
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        if guess.trim() == "quit".to_string() {
            std::process::exit(0);
        }

        // Convert guess to an int
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!\n");
                break;
            },
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Less => println!("Too small!\n"),
        }
    }
}
