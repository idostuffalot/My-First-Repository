use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::Instant;

//crates are a collection of Rust source code files

/*
Question mark: This code does not compile
Two arms up: The code panics
One arm up: This code does not produce the desired behavior
*/

/*
* cmp: compare
* std: standard
* rng: random number generator
* io: input/output

* u32: unsigned (only positive integers)
* i32: signed (positive and negative, but range is split between negative and positive)
* f32: decimals
...and so on (32-64-128)

*/
//hello
fn main() {
    println!("Guess the number!");
    //output << generate random value << generate range (range)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");
    
    let mut last_guess: u32 = 0;
    let mut guess: String;
    let mut guesses_taken: u8 = 0;
    let start: Instant = Instant::now();
    //Loop start (with an infinite loop, press ctrl c to exit)
    loop {

        println!("\n\nPlease input your guess...\n");
        //print the last guess before taking the new input

        //reset the string each time, otherwise it will continue appending
        guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guesses_taken += 1;
        
        /*
        * old method:
        * guess is trimmed of whitespace >> then parsed >> if failed? >> panic >> else it 
        is converted to u32 >> assigned to guess
        */
        let guess: u32 = match guess.trim().parse() {
            //if parse returns an Ok(with the number), it is directed to return num
            Ok(num) => num,
            Err(_) => {               
                println!("invalid input");
                println!("{}", guess.trim());
                continue;
            }
        };
        let elapsed = start.elapsed();
        let seconds: f64 = elapsed.as_millis() as f64/1000.0;

        if last_guess > 0 {println!("Last Guess: {}", last_guess);};

        println!("You guessed: {}", guess);
        // get the current time
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                let score: f32 = 650.0/(guesses_taken as f32/1.3 + (2.0*seconds as f32).sqrt());
                println!("You win! \n\nGuesses taken: {} \nTime Taken: {:?} seconds \n * * Final Score: {:.2} * * ", guesses_taken, seconds, score); 
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
        if last_guess > 0 && guess < 101 {
            let last_guess_closeness: i32 = last_guess as i32 - secret_number as i32;
            let guess_closeness: i32 = guess as i32 - secret_number as i32;
            let _num: i32 = i32::abs(-1);
        
            if i32::abs(last_guess_closeness) < i32::abs(guess_closeness) {
                println!("You're getting farther from the secret number!");
            };

        };

        //update last_guess
        last_guess = guess;
        //loop end, or if win, break from loop
    }
   

}

