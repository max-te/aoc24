#![allow(dead_code)]

pub struct VecVec<T> {
    lengths: Vec<usize>,
    data: Vec<T>,
}

impl<T> VecVec<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            lengths: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push_from<I: Iterator<Item = T>>(&mut self, values: I) {
        let previous_data_length = self.data.len();
        self.data.extend(values);
        self.lengths.push(self.data.len() - previous_data_length);
    }

    pub fn iter(&self) -> VecVecIter<T> {
        VecVecIter {
            data: self,
            lengths_index: 0,
            data_index: 0,
        }
    }
}

pub struct VecVecIter<'this, T> {
    data: &'this VecVec<T>,
    lengths_index: usize,
    data_index: usize,
}

impl<'this, T> Iterator for VecVecIter<'this, T> {
    type Item = &'this [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.lengths_index < self.data.lengths.len() {
            let length = self.data.lengths[self.lengths_index];
            self.lengths_index += 1;
            let start = self.data_index;
            self.data_index += length;
            Some(&self.data.data[start..self.data_index])
        } else {
            None
        }
    }
}

#[inline]
pub fn parse_2_digits(digits: &[u8]) -> u8 {
    debug_assert!(digits.len() == 2);
    debug_assert!(digits[0].is_ascii_digit() && digits[1].is_ascii_digit());
    (digits[0] - b'0') * 10 + digits[1] - b'0'
}

#[inline]
pub fn parse_digit(digit: &u8) -> u8 {
    debug_assert!(digit.is_ascii_digit());
    digit - b'0'
}

#[inline]
pub fn parse_digits_unchecked(digits: &[u8]) -> i64 {
    // TODO: SIMD?
    // TODO: Generic over output type?
    let mut res: i64 = 0;
    for &digit in digits {
        debug_assert!(
            digit.is_ascii_digit(),
            "{:?} is not an ascii digit",
            char::from_u32(digit as u32)
        );
        res *= 10;
        res += (digit - b'0') as i64;
    }
    res
}

#[inline]
pub fn parse_initial_digits(digits: &[u8]) -> (i64, usize) {
    let mut len = 0;
    let mut res: i64 = 0;
    let mut negative = false;
    for &digit in digits {
        match digit {
            b'-' => negative = true,
            b'0'..=b'9' => {
                res *= 10;
                res += (digit - b'0') as i64;
            }
            _ => break,
        }
        len += 1;
    }
    if negative {
        res = -res;
    };
    (res, len)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MyRange<T>(T, T);

impl<T> MyRange<T> {
    pub fn as_std(self) -> std::ops::Range<T> {
        self.0..self.1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedView<'a, T> {
    pub data: &'a [T],
    pub groups: Vec<MyRange<usize>>,
}

impl<'a, T> GroupedView<'a, T> {
    pub fn of_singletons(data: &'a [T]) -> Self {
        Self {
            data,
            groups: (0..data.len()).map(|i| MyRange(i, i + 1)).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.groups.iter().map(|range| &self.data[range.as_std()])
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn merge_left(&mut self, i: usize) -> usize {
        let dissolved = self.groups.remove(i);
        debug_assert!(self.groups[i - 1].1 == dissolved.0);
        self.groups[i - 1].1 = dissolved.1;
        dissolved.0
    }

    pub fn split(&mut self, i: usize, split_point: usize) {
        debug_assert!(self.groups[i].as_std().contains(&split_point));
        self.groups
            .insert(i + 1, MyRange(split_point, self.groups[i].1));
        self.groups[i].1 = split_point;
    }
}

pub fn first_line_length(input: &[u8]) -> usize {
    input
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap_or(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_split_noop() {
        let things = [1, 2, 3, 4, 5];
        let mut view = GroupedView::of_singletons(&things);
        let orig = view.clone();
        assert_eq!(view.len(), 5);
        let merge_point = view.merge_left(2);
        assert_eq!(view.len(), 4);
        assert_eq!(merge_point, 2);
        view.split(1, merge_point);
        assert_eq!(view.len(), 5);
        assert_eq!(view, orig);
    }
}
