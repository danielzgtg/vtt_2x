use crate::types::timing::atoi::*;
use crate::Timestamp;

pub(crate) fn parse_caption_timing(raw: &[u8; 12]) -> Timestamp {
    assert_eq!(b':', raw[2], "Parse caption ':' 1");
    assert_eq!(b':', raw[5], "Parse caption ':' 2");
    assert_eq!(b'.', raw[8], "Parse caption '.' 3");
    Timestamp::new(
        ((atoi2((&raw[0..2]).try_into().unwrap()) * 60 + atoi2((&raw[3..5]).try_into().unwrap()))
            * 60
            + atoi2((&raw[6..8]).try_into().unwrap()))
            * 1000
            + atoi3((&raw[9..12]).try_into().unwrap()),
    )
}
