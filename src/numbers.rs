pub fn nth_digit(i: u32, n: u8, len: u8, radix: u8) -> u8 {
    ((i / (radix as u32).pow((len - n - 1) as u32)) % 10) as u8
}

pub fn log_b(n: f32, b: f32) -> f32 {
    n.ln() / b.ln()
}

pub struct DigitIterator {
    number: u32,
    digit: u8,
    len: u8,
    radix: u8,
}

impl Iterator for DigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digit >= self.len {
            None
        } else {
            let digit = self.digit;
            self.digit += 1;
            Some(nth_digit(self.number, digit, self.len, self.radix))
        }
    }
}

pub struct ReverseDigitIterator {
    number: u32,
}

impl Iterator for ReverseDigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let digit = (self.number % 10) as u8;
        self.number /= 10;
        Some(digit)
    }
}

pub trait DigitIterable {
    fn digits(&self) -> DigitIterator;
    fn reverse_digits(&self) -> ReverseDigitIterator;
}

impl DigitIterable for u32 {
    fn digits(&self) -> DigitIterator {
        DigitIterator::of(*self)
    }

    fn reverse_digits(&self) -> ReverseDigitIterator {
        ReverseDigitIterator { number: *self }
    }
}

impl DigitIterator {
    pub fn of(number: u32) -> DigitIterator {
        DigitIterator {
            number,
            digit: 0,
            len: log_b(number as f32, 10_f32).ceil() as u8,
            radix: 10,
        }
    }
    pub fn with_len(&self, len: u8) -> DigitIterator {
        DigitIterator {
            number: self.number,
            digit: 0,
            len,
            radix: self.radix,
        }
    }
    pub fn with_radix(&self, radix: u8) -> DigitIterator {
        DigitIterator {
            number: self.number,
            digit: 0,
            len: log_b(self.number as f32, radix as f32).ceil() as u8,
            radix,
        }
    }
}
