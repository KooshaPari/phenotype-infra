//! Tab completion for CLI

pub struct CompletionEngine;

impl CompletionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn complete(&self, _input: &str) -> Vec<String> {
        vec![]
    }
}
