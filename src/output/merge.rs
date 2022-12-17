use std::iter::Peekable;
use std::vec::IntoIter;

use crate::types::{Choreographer, Timed};
use crate::verifier::VerifiedHtml;
use crate::{Caption, Chat};

pub(super) struct ChatMerge<'a> {
    choreographer: Choreographer,
    chat: Peekable<IntoIter<Chat<'a>>>,
    captions: Peekable<IntoIter<Caption<'a>>>,
}

impl<'a> ChatMerge<'a> {
    pub(super) fn from(html: VerifiedHtml<'a>, choreographer: Choreographer) -> Self {
        let html = html.html();
        ChatMerge {
            choreographer,
            chat: html.chat.into_iter().peekable(),
            captions: html.captions.into_iter().peekable(),
        }
    }

    fn emit_caption<'b>(&self, caption: Caption<'b>) -> Caption<'b> {
        caption.with_adjusted_time(&self.choreographer)
    }

    fn emit_chat<'b>(&self, chat: Chat<'b>) -> Caption<'b> {
        chat.to_caption(&self.choreographer)
    }
}

impl<'a> Iterator for ChatMerge<'a> {
    type Item = Caption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.chat.peek();
        let caption = self.captions.peek();
        let line = if let Some(line) = line {
            line
        } else {
            return self.captions.next().map(|x| self.emit_caption(x));
        };
        let caption = if let Some(caption) = caption {
            caption
        } else {
            return self.chat.next().map(|x| self.emit_chat(x));
        };
        Some(if line.time() < caption.time() {
            let chat = self.chat.next().unwrap();
            self.emit_chat(chat)
        } else {
            let captions = self.captions.next().unwrap();
            self.emit_caption(captions)
        })
    }
}
