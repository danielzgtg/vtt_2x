use crate::types::timing::atoi::*;
use crate::Timestamp;

fn parse_short_chat_timing(raw: &[u8; 5]) -> Timestamp {
    assert_eq!(b':', raw[2], "Parse short chat ':'");
    Timestamp::new(
        (atoi2((&raw[0..2]).try_into().unwrap()) * 60 + atoi2((&raw[3..5]).try_into().unwrap()))
            * 1000,
    )
}

fn parse_long_chat_timing(raw: &[u8; 8]) -> Timestamp {
    assert_eq!(b':', raw[2], "Parse long chat ':' 1");
    assert_eq!(b':', raw[5], "Parse long chat ':' 2");
    Timestamp::new(
        ((atoi2((&raw[0..2]).try_into().unwrap()) * 60 + atoi2((&raw[3..5]).try_into().unwrap()))
            * 60
            + atoi2((&raw[6..8]).try_into().unwrap()))
            * 1000,
    )
}

pub(crate) fn parse_chat_timing(raw: &[u8]) -> Timestamp {
    match raw.len() {
        5 => parse_short_chat_timing(raw.try_into().unwrap()),
        8 => parse_long_chat_timing(raw.try_into().unwrap()),
        l => panic!("Unexpected chat timing length {}", l),
    }
}
