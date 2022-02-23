use std::ops::Range;

/// A host-owned, sized collection.
pub trait Collection {
    fn len(&self) -> u32;

    fn is_empty(&self) -> bool {
        self.len() != 0
    }
}

/// A host-owned, ordered, sized collection of items.
pub trait Sequence: Collection + Sized {
    /// The type of the items contained in the sequence.
    type Item: Sized + 'static;

    /// Returns the item at the given `index`, or `None` if the index is out of bounds.
    fn get(&self, index: u32) -> Option<Self::Item>;

    /// Returns the first item in the sequence, or `None` if the sequence is empty.
    fn first(&self) -> Option<Self::Item> {
        self.get(0)
    }

    /// Returns the last item in the sequence, or `None` if the sequence is empty.
    fn last(&self) -> Option<Self::Item> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    /// Returns an iterator for the items in the sequence.
    ///
    /// # A word of caution on modification during iteration
    ///
    /// Many host-owned sequence can be modified during iteration. While this will never result in
    /// undefined behavior, it may cause unexpected results. For example, one might expect the
    /// following example to disconnect all of the element's child nodes from the DOM tree:
    ///
    /// ```
    /// for node in element.child_nodes().iter() {
    ///     node.disconnect()
    /// }
    /// ```
    ///
    /// However, at the end of this loop, only nodes at even indices will have been disconnected,
    /// nodes at odd indices will remain connected.
    ///
    /// To achieve the desired result in the above example, one might for example do the following:
    ///
    /// ```
    /// let child_nodes = element.child_nodes();
    ///
    /// while let Some(node) = child_nodes.last() {
    ///     node.disconnect()
    /// }
    /// ```
    ///
    /// In general, caution is advised when modifying a sequence during iteration.
    ///
    /// # A note on `Iterator::size_hint`
    ///
    /// For sequences that are modifyable, the size hint may not remain accurate if the sequence is
    /// modified during iteration. The size hint is always accurate if the sequence is never
    /// modified during iteration.
    fn iter(&self) -> SequenceIter<Self> {
        SequenceIter {
            sequence: self,
            range: 0..self.len(),
        }
    }

    /// Returns a copy of the sequence in host-owned memory as a [js_sys::Array].
    fn to_host_array(&self) -> js_sys::Array;

    /// Returns a [Vec] in linear memory where each element in the sequence has been copied into the
    /// [Vec] in sequence order.
    ///
    /// Note that especially for long sequences, [to_host_array] may offer significantly better
    /// performance.
    fn to_vec(&self) -> Vec<Self::Item> {
        let mut vec = Vec::with_capacity(self.len() as usize);

        let mut i = 0;

        while let Some(item) = self.get(i) {
            vec.push(item);

            i += 1;
        }

        vec
    }
}

/// An iterator over a host-owned sequence.
///
///
#[derive(Clone)]
pub struct SequenceIter<'a, T> {
    sequence: &'a T,
    range: Range<u32>,
}

impl<'a, T> Iterator for SequenceIter<'a, T>
where
    T: Sequence,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;

        self.sequence.get(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

// TODO: decide whether these are appropriate for an iterator over a collection that can be modified
// during iteration (as many host-owned collections can be).
//
// impl<'a, T> DoubleEndedIterator for SequenceIter<'a, T> where T: Sequence {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         let index = self.range.next_back()?;
//
//         Some(self.sequence.get(index))
//     }
// }
//
// impl<'a, T> FusedIterator for SequenceIter<'a, T> where T: Sequence {}
//
// impl<'a, T> ExactSizeIterator for SequenceIter<'a, T> where T: Sequence {}
