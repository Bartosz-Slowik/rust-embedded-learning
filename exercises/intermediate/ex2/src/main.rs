use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let entries = fs::read_dir(args[1].to_string()).unwrap();
    let mut ratings: Vec<(PathBuf, u32)> = Vec::new();

    for entry in entries {
        let path = entry.expect("").path();
        println!("Name: {}", path.display());
        let contents = fs::read_to_string(&path).expect("ERROR FILE");
        clear_terminal();
        println!("{contents}");
        println!("Rate file.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        ratings.push((path, guess.trim().parse::<u32>().unwrap()));
    }
    show_summary(ratings)
}
fn show_summary(ratings: Vec<(PathBuf, u32)>) {
    clear_terminal();
    println!("Summary: ");
    for x in ratings {
        println!("{:20} -> {:2} ", x.0.display(), x.1);
    }
}
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
