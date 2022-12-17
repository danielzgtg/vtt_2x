use crate::{Caption, Chat, Timestamp};

pub(super) fn verify_chat_order(chat: &[Chat]) {
    let mut prev = Timestamp::default();
    for c in chat {
        let cur = c.time();
        assert!(cur >= prev, "Chat time went backwards");
        prev = cur;
    }
}

pub(super) fn verify_caption_order(captions: &[Caption]) {
    let mut prev = Timestamp::default();
    for c in captions {
        let cur = c.time();
        assert!(cur >= prev, "Caption time went backwards");
        prev = cur;
    }
}
