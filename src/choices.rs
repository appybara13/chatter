use crate::{ChatContent, ChatPosition, Tagged};

pub struct Choice {
    text: &'static str,
    tags: &'static [&'static str],
    next: ChatPosition,
}

impl Choice {
    pub fn get_text(&self) -> &'static str {
        self.text
    }

    pub fn get_next(&self) -> ChatPosition {
        self.next
    }

    pub fn new(text: &'static str, tags: &'static [&'static str], next: Option<usize>) -> Choice {
        Choice {
            text,
            tags,
            next: ChatPosition(next),
        }
    }

    pub fn new_group(choices: Vec<Choice>) -> ChatContent {
        ChatContent::Choices(choices)
    }
}

impl Tagged for Choice {
    fn get_tag_names(&self) -> &'static [&'static str] {
        self.tags
    }
}
