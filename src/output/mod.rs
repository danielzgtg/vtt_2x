use crate::output::merge::ChatMerge;
use crate::types::Choreographer;
use crate::VerifiedHtml;

mod merge;

pub(crate) const WEBVTT_HEADER: &'static str = "WEBVTT\n\n";

pub(crate) fn output_vtt(html: VerifiedHtml, choreographer: Choreographer) {
    let mut buffer = String::with_capacity(html.max_size());
    buffer.push_str(WEBVTT_HEADER);
    let vtt = ChatMerge::from(html, choreographer);
    for caption in vtt {
        caption.write(&mut buffer)
    }
    println!("{}", buffer);
}
