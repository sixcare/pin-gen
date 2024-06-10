use std::{
    env, fs::File, io::{prelude::*, BufReader}, path::Path
};
use rand::prelude::*;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
 }

fn main()  {

    let args: Vec<String> = env::args().collect();

    let mut pin_length: u8 = 4;
    let mut pin_amount: u8 = 1;

    // Try index
    match args.get(1) {
        Some(arg) => {
            match arg.parse::<u8>() {
                Ok(value) => {
                    pin_length = value;
                }
                Err(_) => {
                    println!("Error: The first argument, pin length is not an u8 value.");
                    std::process::exit(0)
                }
            }
        }
        None => {
            println!("No pin length provided. Falling back to default(4).");
        }
    }

    match args.get(2) {
        Some(arg) => {
            match arg.parse::<u8>() {
                Ok(value) => {
                    pin_amount = value;
                }
                Err(_) => {
                    println!("Error: The first argument, pin length is not an u8 value.");
                    std::process::exit(0)
                }
            }
        }
        None => {
            println!("No number of pins provided. Falling back to default(1)");
        }
    }

    println!("PIN length is: {}", pin_length);
    println!("PINs to generate: {}", pin_amount);

    let lines = lines_from_file("./charmap.txt");

    let mut char_map: Vec<(u8, char)> = Vec::new();

    for line in lines.iter() {
        if line.len() != 5 {
            println!("Charmap error. Line length should be 5. Like 'A = 0'");
            std::process::exit(0)
        }

        let char_index: u8;
        let s: &str = line.split(" = ").collect::<Vec<_>>()[1];

        match s.parse::<u8>() {
            Ok(v) => {
                char_index = v;
            }
            Err(e) => {
                println!("Conversion failed: {}", e);
                std::process::exit(0)
            }
        }

        char_map.push((
            char_index,
            line.split(" = ").collect::<Vec<_>>()[0].chars().next().unwrap(),

        ));
    }

    let mut words: Vec<String> = lines_from_file("./wordlist.txt");
    words.retain(|x| x.len() == pin_length.into());

    let mut rng = thread_rng();

    for i in 0..pin_amount {
        println!();
        println!("Pin number {}", i+1);
        let mut pin: Vec<(char, u8)> = Vec::new();
    
        let word = words.choose(&mut rng);

        match word {
            Some(result) => {
                let word_upper = result.to_uppercase();
                println!("Word: {}", word_upper);
                for c in word_upper.chars() {
                    for k in &char_map {
                        if k.1 == c {
                            pin.push((k.1, k.0));
                        }
                    }
                }
            },
            None => {
                println!("It fucking failed");
                std::process::exit(0)
            },
        };
        print!("PIN: ");
        for n in &pin {
            print!("{}", n.1);
        }
        println!("");
        println!("Combo: {:?}", pin);
    }


    println!("");

    println!("Character Map:");
    println!(" +===+===+===+===+===+===+===+===+===+===+");
    println!(" | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |");
    println!(" +===+===+===+===+===+===+===+===+===+===+");
    let mut char_vec: Vec<Vec<char>> = Vec::new();
    let mut v: Vec<char> = Vec::new();
    for (i, k) in char_map.iter().enumerate() {
        if k.0 == 0 {
            v.clear()
        }
        v.push(k.1);
        if k.0 == 9 || i == char_map.len() - 1 {
            char_vec.push(v.clone());
        }
    }

    for l in char_vec {
        for k in &l {
            print!(" | {}", k);
        }
        if l.len() <= 9 {
            for _i in 0..9-l.len()+1 {
                    print!(" |  ")
                }
            }
        println!(" |");
    }
    println!(" +---+---+---+---+---+---+---+---+---+---+");


}
