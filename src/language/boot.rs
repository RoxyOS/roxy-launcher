use crate::language::LangHelper;

impl LangHelper {
    pub fn init() -> LangHelper {
        let result = LangHelper::default();
        tokio::spawn(async move {});

        result
    }
}
