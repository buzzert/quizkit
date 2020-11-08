extern crate quizkit;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;

use quizkit::quiz::*;
use std::string::String;
use std::collections::HashMap;
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

type NatoAnswer = String;
struct NatoQuiz;

impl Quiz<NatoAnswer> for NatoQuiz {
    fn generate_prompt(&self) -> Prompt<NatoAnswer> {
        let mut rng = rand::thread_rng();    
        let random_key = ALPHABET.keys().choose(&mut rng).unwrap();
        
        Prompt {
            question: random_key.to_string(), 
            answer:   ALPHABET[random_key].to_string()
        }
    }

    fn check_answer_string(&self, prompt: &Prompt<NatoAnswer>, answer: &str) -> bool {
        return prompt.answer == answer
    }

    fn get_answer_string(&self, prompt: &Prompt<NatoAnswer>) -> String {
        return prompt.answer.clone()
    }
}

fn main() {
    let quiz = NatoQuiz{};
    quiz.run_quiz();
}
