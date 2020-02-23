use serde::{Serialize, Deserialize};
use dialoguer::{Input, Select, Editor};
use dialoguer::theme::ColorfulTheme;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "kind", content = "choices")]
pub enum Answer<T> {
    Select(Vec<T>),
    Input,
    Editor
}

#[derive(Deserialize, Clone, Debug)]
pub struct Question<T> {
    pub name: String,
    pub answer: Answer<T>,
    pub prompt: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response<T> {
    Choice(T),
    Text(String),
}

impl<'a, T: ToString + Clone + Copy> Question<T> {
    pub fn new(name: &str, answer: Answer<T>, prompt: &str) -> Self {
        Question { name: name.to_string(), answer, prompt: prompt.to_string() }
    }

    pub fn ask(&self) -> Response<T> {
        match &self.answer {
            Answer::Select(choices) => Response::Choice(self.render_select(choices.to_vec())),
            Answer::Input => Response::Text(self.render_input()),
            Answer::Editor => Response::Text(self.render_editor()),
        }
    }

    fn render_select(&self, choices: Vec<T>) -> T {
        let theme = ColorfulTheme::default();
        let mut select = Select::with_theme(&theme);

        let answer = select.with_prompt(self.prompt.as_str()).items(&choices[..]).default(2).interact().unwrap();
        choices.get(answer).unwrap().clone()
    }

    fn render_input(&self) -> String {
        let theme = ColorfulTheme::default();
        let mut input = Input::<String>::with_theme(&theme);
        input.with_prompt(self.prompt.as_str()).interact().unwrap()
    }

    fn render_editor(&self) -> String {
        Editor::new().require_save(false).edit(self.prompt.as_str()).unwrap().unwrap()
    }
}
