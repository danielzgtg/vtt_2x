use crate::{Caption, Timestamp};

pub(crate) struct Chat<'a> {
    time: Timestamp,
    text: &'a str,
    username: &'a str,
}

impl<'a> Chat<'a> {
    pub(crate) fn new(time: Timestamp, text: &'a str, username: &'a str) -> Chat<'a> {
        assert!(!text.contains("\n"), "Newline in chat");
        assert!(!text.is_empty(), "Empty chat");
        assert!(!username.contains("\n"), "Newline in username");
        assert!(!username.is_empty(), "Empty username");
        Chat {
            time,
            text,
            username,
        }
    }

    pub(crate) fn time(&self) -> Timestamp {
        self.time
    }

    pub(crate) fn adjust_time(&mut self) {
        self.time.halve();
    }

    pub(crate) fn max_size(&self) -> usize {
        Caption::max_size_internal(self.text, self.username)
    }

    pub(crate) fn to_caption(self) -> Caption<'a> {
        let start = self.time;
        let end = {
            let mut end = start.clone();
            end.add_5sec();
            end
        };
        Caption::new(start, end, self.text, Some(self.username))
    }
}
