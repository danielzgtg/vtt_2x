use crate::{parse_caption_timing, Caption};
use std::iter::Peekable;
use std::str::Lines;

#[inline]
fn parse_subtitles_impl<'a, const CC: bool, const VTT: bool>(
    subtitles: &mut Peekable<Lines<'a>>,
    result: &mut Vec<Caption<'a>>,
) {
    let mut last_header = 0u32;
    loop {
        let header = if let Some(header) = subtitles.next() {
            if header.is_empty() {
                break;
            }
            header
        } else {
            break;
        };
        let timings = if CC {
            header
        } else {
            let parsed_header = header.parse::<u32>().expect("Parsing caption id");
            assert_eq!(
                parsed_header,
                last_header + 1,
                "Caption IDs not strictly increasing"
            );
            last_header = parsed_header;
            subtitles.next().expect("Expected timings after caption id")
        };
        if timings.is_empty() {
            break;
        }
        let timings: &[u8; 29] = timings
            .as_bytes()
            .try_into()
            .expect("Transcript timings corrupt length");
        assert_eq!(
            &timings[12..17],
            Caption::VTT_SEPARATOR_BYTES,
            "Transcript timing separator wrong"
        );
        let start = parse_caption_timing::<VTT>((&timings[0..12]).try_into().unwrap());
        let end = parse_caption_timing::<VTT>((&timings[17..29]).try_into().unwrap());
        let text = subtitles.next().expect("Transcript text line");
        let text2 = subtitles.next().unwrap_or("");
        result.push(Caption::new(start, end, text, text2, None));
        if !text2.is_empty() {
            if let Some(blank) = subtitles.next() {
                assert!(blank.is_empty(), "Transcript blank line, is not");
            } else {
                break;
            }
        }
    }
}

#[inline]
fn parse_subtitles<const CC: bool>(subtitles: &str) -> Vec<Caption> {
    let mut result = Vec::with_capacity(subtitles.lines().count() / 3);
    let mut subtitles = subtitles.lines().peekable();
    if matches!(subtitles.peek(), Some(&"WEBVTT")) {
        subtitles.next().unwrap();
        assert!(matches!(subtitles.next(), Some("")), "After VTT Header");
        parse_subtitles_impl::<CC, true>(&mut subtitles, &mut result);
    } else {
        parse_subtitles_impl::<CC, false>(&mut subtitles, &mut result);
    }
    assert!(
        matches!(subtitles.next(), None),
        "Transcript unexpected trailing data"
    );
    result
}

pub(crate) fn parse_cc(vtt: &str) -> Vec<Caption> {
    parse_subtitles::<true>(vtt)
}

pub(crate) fn parse_transcript(vtt: &str) -> Vec<Caption> {
    parse_subtitles::<false>(vtt)
}
