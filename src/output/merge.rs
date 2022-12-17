use std::iter::Peekable;
use std::vec::IntoIter;

use crate::verifier::VerifiedHtml;
use crate::{Caption, Chat};

pub(super) struct ChatMerge<'a> {
    chat: Peekable<IntoIter<Chat<'a>>>,
    captions: Peekable<IntoIter<Caption<'a>>>,
}

impl<'a> ChatMerge<'a> {
    pub(super) fn from(html: VerifiedHtml<'a>) -> Self {
        let html = html.html();
        ChatMerge {
            chat: html.chat.into_iter().peekable(),
            captions: html.captions.into_iter().peekable(),
        }
    }
}

fn emit_caption(mut caption: Caption) -> Caption {
    caption.adjust_time();
    caption
}

fn emit_chat(mut chat: Chat) -> Caption {
    chat.adjust_time();
    chat.to_caption()
}

impl<'a> Iterator for ChatMerge<'a> {
    type Item = Caption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.chat.peek();
        let caption = self.captions.peek();
        let line = if let Some(line) = line {
            line
        } else {
            return self.captions.next().map(emit_caption);
        };
        let caption = if let Some(caption) = caption {
            caption
        } else {
            return self.chat.next().map(emit_chat);
        };
        Some(if line.time() < caption.time() {
            emit_chat(self.chat.next().unwrap())
        } else {
            emit_caption(self.captions.next().unwrap())
        })
    }
}
