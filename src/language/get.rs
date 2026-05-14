use crate::language::LangHelper;

impl LangHelper {
    pub fn get_text<'a>(&'a self, prompt: &'a str) -> &'a str {
        match self.all_lang_resouce.get(&self.current_lang) {
            Some(s) => match s.get(prompt) {
                Some(result) => result,
                None => prompt,
            },
            None => prompt,
        }
    }
}
