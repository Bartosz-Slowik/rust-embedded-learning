use std::path::PathBuf;
use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut oceny =  Vec::new();

    dbg!(&args);

    let entries = fs::read_dir(args[1].to_string()).unwrap();

    for entry in entries {
        let path = entry.expect("").path();
        println!("Name: {}", path.display());
        
        let contents = fs::read_to_string(path)
            .expect("ERROR PLIKU");
        println!("{contents}");
        println!("Oceń plik.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            oceny.push(guess.trim().parse::<u32>().unwrap());
            
    }
    dbg!(oceny);

    
    


    //let dir_content = todo!("get contents of the directory");

    // `Vec<(PathBuf, u32)>` is a dynamically sized array
    // of tuples containing `PathBuf` and `u32`
   /* let ratings: Vec<(PathBuf, u32)> = todo!("
        for entry in dir_content
            clear the screen
            present contents of a file to the user
            ask user to input his rating from a 1-5 scale and store it in `ratings`,
    ");
    */
    
    //todo!("Present the ratings summary to the user");
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
