extern crate termion;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use termion::{color, style};
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use rand::seq::IteratorRandom;

lazy_static! {
    static ref ALPHABET: HashMap<&'static str, &'static str> = hashmap! {
        "a" => "alpha",
        "b" => "bravo",
        "c" => "charlie", 
        "d" => "delta",
        "e" => "echo",
        "f" => "foxtrot",
        "g" => "golf",
        "h" => "hotel",
        "i" => "india",
        "j" => "juliett", 
        "k" => "kilo",
        "l" => "lima",
        "m" => "mike",
        "n" => "november",
        "o" => "oscar", 
        "p" => "papa",
        "q" => "quebec",
        "r" => "romeo",
        "s" => "sierra",
        "t" => "tango",
        "u" => "uniform",
        "v" => "victor",
        "w" => "whiskey",
        "x" => "x-ray",
        "y" => "yankee",
        "z" => "zulu",
    };
}

struct Prompt {
    question: &'static str,
    answer:   &'static str
}

fn generate_prompt() -> Prompt {
    let mut rng = rand::thread_rng();    
    let random_key = ALPHABET.keys().choose(&mut rng).unwrap();
    
    Prompt {
        question: random_key, 
        answer:   ALPHABET[random_key]
    }
}

fn main() {
    let mut stdout = stdout();

    let stdin = stdin();
    let mut stdin = stdin.lock();

    loop {
        let prompt = generate_prompt();
        
        let mut answer_correct: bool = false;
        while answer_correct == false {
            // Write prompt
            let prompt_str = format!("{:?}: ", prompt.question);
            print!("{}", prompt_str);
            let _ = stdout.flush();

            // Read answer
            let answer_opt = stdin.read_line();

            if let Some(answer) = answer_opt.unwrap() {
                // Enter "?" to get the answer 
                if answer == "?" {
                    println!("{}Answer:{} {}", style::Bold, style::Reset, prompt.answer);
                    answer_correct = true;
                } else {
                    // Move cursor to end of line to provide result
                    let backtrack_len = (answer.len() + prompt_str.len()) as u16;
                    print!("{}{}", termion::cursor::Up(1), termion::cursor::Right(backtrack_len + 2));

                    answer_correct = answer == prompt.answer;
                    if answer_correct {
                        print!("{}✓", color::Fg(color::LightGreen));
                    } else {
                        print!("{}x", color::Fg(color::Red));
                    }

                    print!("{}\n", style::Reset);
                }
            }
        }
    }
}
