use crate::Caption;

#[allow(unused)]
pub(super) fn verify_against_cc(cc: Vec<Caption>, html: &[Caption]) {
    for (cc, html) in cc.into_iter().zip(html) {
        html.verify_cc(&cc);
    }
}

#[allow(unused)]
pub(super) fn verify_against_transcript(transcript: Vec<Caption>, html: &[Caption]) {
    for (transcript, html) in transcript.into_iter().zip(html) {
        html.verify_transcript(&transcript);
    }
}
