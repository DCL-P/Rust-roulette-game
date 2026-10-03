use std::io;
use create::players;

//[?] apperently u need to specify the return type when u use something like ? after statements (like on line 17)
pub fn game_loop()-> Result<(), Box<dyn std::error::Error>> {

    //defined stdin for easy read lining
    let stdin = io::stdin();


    loop {
        println!("would you like to play?");

        let mut answer = String::new();

        //[!] read_line returns a Result, which can return an Ok() or Err() object. So make sure if u want to print the value, trim it first to unwrap out of the Ok or Err object
        //[!] ? basically just means (if value is Ok, continue. Err? stop!)
        stdin.read_line(&mut answer)?;


        //[!] in this case trim removes the \n newline from answer, 
        if answer.trim() == "YES"{
            println!("cool!");
            println!("please enter your name");

            let mut username = String::new();

            stdin.read_line(&mut username);

            println!("your selected user is: {}", username);
        }
        else{
            println!("SAY YES RIGHT NOW!!!!");
        }
    }

}
