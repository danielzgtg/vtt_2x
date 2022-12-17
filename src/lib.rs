use crate::filesystem::{open_files, OpenFiles};
use crate::output::output_vtt;
use crate::parser::{parse_cc, parse_html, parse_transcript, Html};
pub use crate::types::Choreographer;
use crate::types::{parse_caption_timing, parse_chat_timing, Caption, Chat, Timed, Timestamp};
use crate::verifier::VerifiedHtml;

mod filesystem;
mod output;
mod parser;
mod types;
mod verifier;

pub fn run(paths: Vec<String>, choreographer: Choreographer) {
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
    let parsed_html = if let Some(html) = html.as_deref() {
        parse_html(&html)
    } else {
        Html::from_bypass(
            parsed_transcript
                .as_deref()
                .expect("Need either html or cc")
                .to_vec(),
        )
    };
    let verified_html = VerifiedHtml::verify(parsed_cc, parsed_transcript, parsed_html);
    output_vtt(verified_html, choreographer);
}
