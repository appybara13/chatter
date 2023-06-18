use crate::{ChatContent, ChatPosition, Tagged};

pub struct Line {
    text: &'static str,
    tags: &'static [&'static str],
    next: ChatPosition,
}

impl Line {
    pub fn get_text(&self) -> &'static str {
        self.text
    }

    pub fn get_next(&self) -> ChatPosition {
        self.next
    }

    pub fn new(
        text: &'static str,
        tags: &'static [&'static str],
        next: Option<usize>,
    ) -> ChatContent {
        ChatContent::Line(Line {
            text,
            tags,
            next: ChatPosition(next),
        })
    }
}

impl Tagged for Line {
    fn get_tag_names(&self) -> &'static [&'static str] {
        self.tags
    }
}
