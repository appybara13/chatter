use crate::{Choice, Line};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChatPosition(pub(crate) Option<usize>);

impl ChatPosition {
    pub fn end() -> ChatPosition {
        ChatPosition(None)
    }

    pub fn start() -> ChatPosition {
        ChatPosition(Some(0))
    }
}

pub enum ChatContent {
    Line(Line),
    Choices(Vec<Choice>),
}

pub struct Chat {
    content: Vec<ChatContent>,
}

impl Chat {
    pub fn get(&self, position: ChatPosition) -> Option<&ChatContent> {
        match position.0 {
            Some(pos) => Some(&self.content[pos]),
            None => None,
        }
    }

    pub fn new(content: Vec<ChatContent>) -> Chat {
        Chat { content }
    }
}
