use crate::Timestamp;
use std::cmp::max;

pub(crate) struct Caption<'a> {
    start: Timestamp,
    end: Timestamp,
    text: &'a str,
    username: Option<&'a str>,
}

/// Unescapes the JSON string, only allowing resonable escapes
fn unescape_json_safely(text: &str, result: &mut String) {
    let mut text = text.chars();
    while let Some(c) = text.next() {
        if c != '\\' {
            if c == '"' {
                panic!("Naked double quote in JSON string");
            }
            result.push(c);
            continue;
        }
        let c = text.next().expect("Unterminated JSON escape sequence");
        match c {
            '\'' | '"' | '\\' | '>' => result.push(c),
            'x' => {
                assert!(matches!(text.next(), Some('3')), "Expected angle escape 3");
                assert!(matches!(text.next(), Some('C')), "Expected angle escape C");
                result.push('<');
            }
            _ => panic!("Unexpected JSON escape sequence {}", c),
        }
    }
}

impl<'a> Caption<'a> {
    pub(crate) fn new(
        start: Timestamp,
        end: Timestamp,
        text: &'a str,
        username: Option<&'a str>,
    ) -> Caption<'a> {
        assert!(!text.contains("\n"), "Newline in caption");
        assert!(!text.is_empty(), "Empty caption");
        if let Some(username) = username {
            debug_assert!(!username.contains("\n"));
            debug_assert!(!username.is_empty());
        }
        Caption {
            start,
            end,
            text,
            username,
        }
    }

    pub(crate) fn time(&self) -> Timestamp {
        self.start
    }

    pub(crate) fn adjust_time(&mut self) {
        self.start.halve();
        self.end = max(
            {
                let mut x = self.start.clone();
                x.add_2sec();
                x
            },
            {
                let mut x = self.end.clone();
                x.halve();
                x
            },
        );
    }

    //noinspection RsAssertEqual
    /// Verifies closed captions assuming self is from HTML
    pub(crate) fn verify_cc(&self, cc: &Caption) {
        debug_assert!(self.username.is_some());
        debug_assert!(cc.username.is_none());
        assert!(self.start == cc.start);
        assert!(self.end == cc.end);
        assert!(self.text == cc.text);
    }

    //noinspection RsAssertEqual
    /// Verifies a transcript assuming self is from HTML
    pub(crate) fn verify_transcript(&self, cc: &Caption) {
        let username = self.username.unwrap();
        debug_assert!(cc.username.is_none());
        assert!(self.start == cc.start);
        assert!(self.end == cc.end);
        assert!(cc.text.starts_with(username));
        assert!(self.text == &cc.text[username.len()..]);
    }

    const VTT_SEPARATOR_STR: &'static str = " --> ";
    pub(crate) const VTT_SEPARATOR_BYTES: &'static [u8; 5] = b" --> ";

    pub(in crate::types) fn max_size_internal(text_json: &str, username_json: &str) -> usize {
        12 + 1 + username_json.len() + 2 + text_json.len() + 2
    }

    pub(crate) fn max_size(&self) -> usize {
        Self::max_size_internal(self.text, self.username.unwrap())
    }

    pub(crate) fn write(&self, result: &mut String) {
        let username = self.username.unwrap();
        let mut buf = [0u8; 12];
        self.start.vtt(&mut buf);
        result.push_str(std::str::from_utf8(&buf[..]).unwrap());
        result.push_str(Self::VTT_SEPARATOR_STR);
        self.end.vtt(&mut buf);
        result.push_str(std::str::from_utf8(&buf[..]).unwrap());
        result.push('\n');
        unescape_json_safely(username, result);
        result.push_str(": ");
        unescape_json_safely(self.text, result);
        result.push_str("\n\n");
    }
}
