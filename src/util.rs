
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

