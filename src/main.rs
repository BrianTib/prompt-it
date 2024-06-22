use prompt_it::{Prompter, Prompt, PromptMode};

fn main() {
    // Example of using a constructor to build a prompt
    // let mut name_prompt = Prompt::new();

    // name_prompt.set_mode(PromptMode::Input)
    //     .set_message("What is your name?")
    //     .set_name("name");

    let questions = vec![
        Prompt {
            mode: PromptMode::Input,
            name: "name".to_string(),
            message: "What is your name?".to_string(),
            validate: None,
        },
        Prompt {
            mode: PromptMode::Input,
            name: "location".to_string(),
            message: "Where are you from?".to_string(),
            validate: None
        },
    ];

    let mut prompter = Prompter::new(questions);

    let answers = prompter.prompt();

    println!("{:#?}", answers);
}
