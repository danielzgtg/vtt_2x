use crate::output::WEBVTT_HEADER;
use crate::verifier::order::{verify_caption_order, verify_chat_order};
use crate::{Caption, Html};

mod consistency;
mod order;

pub(crate) struct VerifiedHtml<'a>(Html<'a>);

impl<'a> VerifiedHtml<'a> {
    pub(crate) fn verify(
        cc: Option<Vec<Caption>>,
        transcript: Option<Vec<Caption>>,
        html: Html<'a>,
    ) -> Self {
        verify_chat_order(&html.chat);
        verify_caption_order(&html.captions);
        if let Some(_cc) = cc {
            // verify_caption_order(&cc); // WTF Zoom?
            // verify_against_cc(cc, &html.captions); // TODO fix or remove
        }
        if let Some(transcript) = transcript {
            verify_caption_order(&transcript);
            // verify_against_transcript(transcript, &html.captions); // TODO fix or remove
        }
        VerifiedHtml(html)
    }

    pub(crate) fn max_size(&self) -> usize {
        let Html { chat, captions } = &self.0;
        let mut result = 0;
        result += WEBVTT_HEADER.len();
        for line in chat {
            result += line.max_size();
        }
        for line in captions {
            result += line.max_size();
        }
        result
    }

    #[inline]
    pub(crate) fn html(self) -> Html<'a> {
        self.0
    }
}
