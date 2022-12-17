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

    #[inline]
    pub(crate) fn halve(&mut self) {
        self.0 /= 2;
    }

    #[inline]
    pub(crate) fn add_5sec(&mut self) {
        self.0 += 5000;
    }

    #[inline]
    pub(crate) fn add_2sec(&mut self) {
        self.0 += 2000;
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
