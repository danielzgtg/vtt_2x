use crate::{Timed, Timestamp};

pub(super) fn verify_order(stream: &[impl Timed]) {
    let mut prev = Timestamp::default();
    for c in stream {
        let cur = c.time();
        assert!(cur >= prev, "Time went backwards");
        prev = cur;
    }
}
