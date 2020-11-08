extern crate quizkit;

use quizkit::quiz::{Quiz, Prompt};
use rand::prelude::*;

type AdditionAnswer = u16;
struct AdditionQuiz;
impl Quiz<AdditionAnswer> for AdditionQuiz {
    fn generate_prompt(&self) -> Prompt<AdditionAnswer> {
        let mut rng = rand::thread_rng();

        let operand1: u16 = rng.gen_range(1, 9);
        let operand2: u16 = rng.gen_range(1, 9);

        Prompt{
            question: format!("{:} + {:}", operand1, operand2),
            answer: (operand1 + operand2)
        }
    }

    fn check_answer_string(&self, prompt: &Prompt<AdditionAnswer>, answer: &str) -> bool {
        match answer.parse::<u16>() {
            Ok(ans) => return prompt.answer == ans,
            Err(_)  => return false
        };
    }

    fn get_answer_string(&self, prompt: &Prompt<AdditionAnswer>) -> std::string::String {
        format!("{:}", prompt.answer)
    }
}

fn main() {
    let quiz = AdditionQuiz{};
    quiz.run_quiz();
}
