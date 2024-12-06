use std::ops::Deref;

#[derive(Debug, Default)]
pub(crate) struct OrderedList {
    pub(crate) list: Vec<i32>,
}

impl From<Vec<i32>> for OrderedList {
    fn from(mut list: Vec<i32>) -> Self {
        list.sort();
        OrderedList { list }
    }
}

impl Extend<i32> for OrderedList {
    fn extend<T: IntoIterator<Item = i32>>(&mut self, iter: T) {
        for item in iter {
            self.list.push(item);
        }
        self.list.sort();
    }
}

impl Deref for OrderedList {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
