pub(super) mod atoi {
    #[inline]
    fn atoi1(a: u8) -> u32 {
        match a {
            b'0'..=b'9' => (a - b'0') as u32,
            _ => panic!("{} not digit", a),
        }
    }

    #[inline]
    pub(in crate::types) fn atoi2(&[a, b]: &[u8; 2]) -> u32 {
        atoi1(a) * 10 + atoi1(b)
    }

    #[inline]
    pub(in crate::types) fn atoi3(&[a, b, c]: &[u8; 3]) -> u32 {
        atoi1(a) * 100 + atoi1(b) * 10 + atoi1(c)
    }
}

#[inline]
fn itoa1(n: u32, result: &mut u8) {
    *result = match n {
        0..=9 => n as u8 + b'0',
        _ => panic!(),
    };
}

#[inline]
fn itoa2(n: u32, result: &mut [u8; 2]) {
    itoa1(n % 10, &mut result[1]);
    itoa1(n / 10, &mut result[0]);
}

#[inline]
fn itoa3(mut n: u32, result: &mut [u8; 3]) {
    itoa1(n % 10, &mut result[2]);
    n /= 10;
    itoa1(n % 10, &mut result[1]);
    itoa1(n / 10, &mut result[0]);
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Timestamp(u32);

impl Timestamp {
    #[inline]
    pub(super) fn new(parsed: u32) -> Self {
        Timestamp(parsed)
    }

    pub(crate) fn vtt(&self, result: &mut [u8; 12]) {
        let mut n = self.0;
        result[2] = b':';
        result[5] = b':';
        result[8] = b'.';
        itoa3(n % 1000, (&mut result[9..12]).try_into().unwrap());
        n /= 1000;
        itoa2(n % 60, (&mut result[6..8]).try_into().unwrap());
        n /= 60;
        itoa2(n % 60, (&mut result[3..5]).try_into().unwrap());
        itoa2(n / 60, (&mut result[0..2]).try_into().unwrap());
    }
}

pub struct Choreographer {
    chat_time: u32,
    min_caption_time: u32,
    caption_divisor: u32,
}

impl Choreographer {
    pub fn new(
        chat_time: Option<u32>,
        min_caption_time: Option<u32>,
        caption_divisor: Option<u32>,
    ) -> Self {
        Choreographer {
            chat_time: chat_time.unwrap_or(5000),
            min_caption_time: min_caption_time.unwrap_or(2000),
            caption_divisor: caption_divisor.unwrap_or(2),
        }
    }

    pub(crate) fn add_chat_time(&self, timestamp: Timestamp) -> Timestamp {
        Timestamp(timestamp.0 + self.chat_time)
    }

    pub(crate) fn adjust_caption_time(&self, start: Timestamp, end: Timestamp) -> [Timestamp; 2] {
        let start = start.0 / self.caption_divisor;
        let end = (start + self.min_caption_time).max(end.0 / self.caption_divisor);
        [Timestamp(start), Timestamp(end)]
    }
}

impl Default for Choreographer {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

pub(crate) trait Timed {
    fn time(&self) -> Timestamp;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vtt_1() {
        let expected = b"00:06:01.000";
        let mut actual = [0u8; 12];
        Timestamp::new(361000).vtt(&mut actual);
        assert_eq!(expected, &actual);
    }
}
