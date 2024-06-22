type TValidateFunction = fn() -> Result<(), String>;

#[derive(Debug)]
pub enum PromptMode {
    Input,
    Number,
}

#[derive(Debug)]
pub struct Prompt {
    pub mode: PromptMode,
    pub message: String,
    pub name: String,
    pub validate: Option<TValidateFunction>
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            mode: PromptMode::Input,
            message: "".to_string(),
            name: "".to_string(),
            validate: None
        }
    }
}

// Builder for prompts
impl Prompt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mode(&mut self, mode: PromptMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_string();
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
}
