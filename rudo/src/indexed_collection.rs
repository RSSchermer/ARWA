// TODO: I feel very unsure about whether this is an appropriate abstraction. For now just duplicate
// implementations for all collection types.

pub trait IndexedCollection {
    type Item;

    fn get(&self, index: usize) -> Option<Self::Item>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    fn first(&self) -> Option<Self::Item> {
        self.get(0)
    }

    fn last(&self) -> Option<Self::Item> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    fn iter(&self) -> IndexedCollectionIter<Self> where Self: Sized {
        IndexedCollectionIter {
            collection: self,
            current: 0
        }
    }
}

pub struct IndexedCollectionIter<'a, T> {
    collection: &'a T,
    current: usize
}

impl<'a, T> Iterator for IndexedCollectionIter<'a, T> where T: IndexedCollection {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.collection.get(current)
    }
}
