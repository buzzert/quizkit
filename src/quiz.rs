extern crate termion;

use termion::{color, style};
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

pub struct Prompt<T> {
    pub question: std::string::String,
    pub answer:   T,
}

pub trait Quiz<T> {
    fn generate_prompt(&self) -> Prompt<T>;
    fn check_answer_string(&self, prompt: &Prompt<T>, answer: &str) -> bool;
    fn get_answer_string(&self, prompt: &Prompt<T>) -> std::string::String;

    fn run_quiz(&self) {
        let mut stdout = stdout();

        let stdin = stdin();
        let mut stdin = stdin.lock();
    
        loop {
            let prompt = &self.generate_prompt();
            
            let mut answer_correct: bool = false;
            while answer_correct == false {
                // Write prompt
                let prompt_str = format!("{:}: ", &prompt.question);
                print!("{}", prompt_str);
                let _ = stdout.flush();
    
                // Read answer
                let answer_opt = stdin.read_line();
    
                if let Some(answer) = answer_opt.unwrap() {
                    // Enter "?" to get the answer 
                    if answer == "?" {
                        let answer_string = self.get_answer_string(prompt);
                        println!("{}Answer:{} {}", style::Bold, style::Reset, answer_string);
                        answer_correct = true;
                    } else {
                        // Move cursor to end of line to provide result
                        let backtrack_len = (answer.len() + prompt_str.len()) as u16;
                        print!("{}{}", termion::cursor::Up(1), termion::cursor::Right(backtrack_len + 2));
    
                        answer_correct = self.check_answer_string(prompt, &answer);
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
}
