use crate::{parse_caption_timing, Caption};

#[inline]
pub(crate) fn parse_vtt<const CC: bool>(vtt: &str) -> Vec<Caption> {
    let mut result = Vec::with_capacity(vtt.lines().count() / 3);
    let mut vtt = vtt.lines();
    assert!(matches!(vtt.next(), Some("WEBVTT")), "VTT Header");
    assert!(matches!(vtt.next(), Some("")), "After VTT Header");
    loop {
        let header = if let Some(header) = vtt.next() {
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
            vtt.next().expect("Expected timings after identifier")
        };
        if timings.is_empty() {
            break;
        }
        let timings: &[u8; 29] = timings
            .as_bytes()
            .try_into()
            .expect("VTT timings corrupt length");
        assert_eq!(
            &timings[12..17],
            Caption::VTT_SEPARATOR_BYTES,
            "VTT timing separator wrong"
        );
        let start = parse_caption_timing((&timings[0..12]).try_into().unwrap());
        let end = parse_caption_timing((&timings[17..29]).try_into().unwrap());
        let text = vtt.next().expect("VTT text line");
        let blank = vtt.next().expect("VTT blank line");
        assert!(blank.is_empty(), "VTT blank line, is not");
        result.push(Caption::new(start, end, text, None));
    }
    assert!(matches!(vtt.next(), None), "VTT unexpected trailing data");
    result
}

pub(crate) fn parse_cc(vtt: &str) -> Vec<Caption> {
    parse_vtt::<true>(vtt)
}

pub(crate) fn parse_transcript(vtt: &str) -> Vec<Caption> {
    parse_vtt::<false>(vtt)
}
