use crate::prompt::Prompt;

use colored::*;
use std::io::{self, Read, Write};
use std::collections::HashMap;

type PromptAnswers = HashMap<String, Option<String>>;
type TValidateFunction = fn() -> Result<(), String>;

pub struct Prompter {
    pub prompts: Vec<Prompt>,
    pub validate: Option<TValidateFunction>,
}

impl Prompter {
    pub fn new(questions: Vec<Prompt>) -> Self {
        Self {
            prompts: questions,
            validate: None,
        }
    }

    /// Prompt the user with all of the prompts
    pub fn prompt(&mut self) -> PromptAnswers {
        let mut answers = HashMap::new();

        for prompt in self.prompts.iter() {
            let answer = _get_console_input(prompt.message.as_str());
            answers.insert(prompt.name.clone(), answer);
        }

        return answers;
    }

    pub fn add_prompt(&mut self, prompt: Prompt) {
        self.prompts.push(prompt);
    }
}

/// Gets input from the console
fn _get_console_input(prompt: &str) -> Option<String> {
    print!("{} {} ", "?".green(), prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim the input to remove any trailing newline characters
            let trimmed = input.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }
        Err(_) => None,
    }
}

// fn _get_console_input_validated(prompt: &str) -> Option<String> {
//     print!("{} {} ", "?".green(), prompt);
//     io::stdout().flush().expect("Failed to flush stdout");


//     io::stdin().read()
// }
