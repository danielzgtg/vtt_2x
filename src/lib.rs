use crate::filesystem::{open_files, OpenFiles};
use crate::output::output_vtt;
use crate::parser::{parse_cc, parse_html, parse_transcript, Html};
use crate::types::{parse_caption_timing, parse_chat_timing, Caption, Chat, Timestamp};
use crate::verifier::VerifiedHtml;

mod filesystem;
mod output;
mod parser;
mod types;
mod verifier;

pub fn run<'a, I: Iterator<Item = &'a str>>(paths: I) {
    let OpenFiles {
        cc,
        transcript,
        html,
    } = open_files(paths);
    let parsed_cc = if let Some(x) = &cc {
        Some(parse_cc(&x))
    } else {
        None
    };
    let parsed_transcript = if let Some(x) = &transcript {
        Some(parse_transcript(x))
    } else {
        None
    };
    let parsed_html = parse_html(&html);
    let verified_html = VerifiedHtml::verify(parsed_cc, parsed_transcript, parsed_html);
    drop(cc);
    drop(transcript);
    output_vtt(verified_html);
}
