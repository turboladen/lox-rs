pub(crate) struct Scanner {
    source: String,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self { source }
    }

    pub(crate) fn scan_tokens(&self) -> Vec<Token> {
        todo!("Implement scan_tokens()")
    }
}

#[derive(Debug)]
pub(crate) struct Token;
