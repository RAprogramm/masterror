// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Inline vector implementation for small collections.
//!
//! This module provides [`InlineVec`], a vector that stores elements inline
//! (on the stack) up to a compile-time capacity, spilling to heap only when
//! that capacity is exceeded. This eliminates heap allocations for the common
//! case of small collections.
//!
//! # Performance
//!
//! For collections that typically contain 0-4 elements (like error metadata),
//! this avoids heap allocation entirely, saving ~100-200ns per error creation.

use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

/// Inline capacity for metadata fields.
///
/// Most errors have 0-4 metadata fields, so we inline up to 4 elements.
const INLINE_CAPACITY: usize = 4;

/// A vector that stores up to 4 elements inline, spilling to heap otherwise.
///
/// This is optimized for the common case where collections are small. When the
/// number of elements exceeds the inline capacity, all elements are moved to
/// a heap-allocated [`Vec`].
///
/// # Examples
///
/// ```ignore
/// let mut vec: InlineVec<i32> = InlineVec::new();
/// vec.push(1);
/// vec.push(2);
/// assert_eq!(vec.len(), 2);
/// assert!(vec.is_inline()); // Still on stack
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlineVec<T> {
    storage: Storage<T>
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Storage<T> {
    /// Inline storage for 0-4 elements using fixed arrays.
    ///
    /// We use separate variants to avoid Option overhead and keep
    /// the common case (0-2 elements) as small as possible.
    Empty,
    One(T),
    Two([T; 2]),
    Three([T; 3]),
    Four([T; 4]),
    /// Heap storage when capacity is exceeded.
    Heap(Vec<T>)
}

impl<T> InlineVec<T> {
    /// Creates a new empty inline vector.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            storage: Storage::Empty
        }
    }

    /// Returns the number of elements in the vector.
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.storage {
            Storage::Empty => 0,
            Storage::One(_) => 1,
            Storage::Two(_) => 2,
            Storage::Three(_) => 3,
            Storage::Four(_) => 4,
            Storage::Heap(vec) => vec.len()
        }
    }

    /// Returns `true` if the vector contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(&self.storage, Storage::Empty)
    }

    /// Returns `true` if elements are stored inline (on stack).
    #[must_use]
    pub fn is_inline(&self) -> bool {
        !matches!(&self.storage, Storage::Heap(_))
    }

    /// Appends an element to the back of the vector.
    ///
    /// If the inline capacity is exceeded, all elements are moved to heap.
    pub fn push(&mut self, value: T) {
        self.storage = match core::mem::take(&mut self.storage) {
            Storage::Empty => Storage::One(value),
            Storage::One(a) => Storage::Two([a, value]),
            Storage::Two([a, b]) => Storage::Three([a, b, value]),
            Storage::Three([a, b, c]) => Storage::Four([a, b, c, value]),
            Storage::Four([a, b, c, d]) => {
                let mut vec = Vec::with_capacity(INLINE_CAPACITY + 1);
                vec.extend([a, b, c, d, value]);
                Storage::Heap(vec)
            }
            Storage::Heap(mut vec) => {
                vec.push(value);
                Storage::Heap(vec)
            }
        };
    }

    /// Returns a reference to the element at the given index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        match &self.storage {
            Storage::Empty => None,
            Storage::One(a) if index == 0 => Some(a),
            Storage::One(_) => None,
            Storage::Two(arr) => arr.get(index),
            Storage::Three(arr) => arr.get(index),
            Storage::Four(arr) => arr.get(index),
            Storage::Heap(vec) => vec.get(index)
        }
    }

    /// Returns a mutable reference to the element at the given index.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.storage {
            Storage::Empty => None,
            Storage::One(a) if index == 0 => Some(a),
            Storage::One(_) => None,
            Storage::Two(arr) => arr.get_mut(index),
            Storage::Three(arr) => arr.get_mut(index),
            Storage::Four(arr) => arr.get_mut(index),
            Storage::Heap(vec) => vec.get_mut(index)
        }
    }

    /// Inserts an element at the specified index, shifting elements after it.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, index: usize, value: T) {
        let len = self.len();
        assert!(index <= len, "insertion index out of bounds");

        self.storage = match core::mem::take(&mut self.storage) {
            Storage::Empty => {
                assert!(index == 0);
                Storage::One(value)
            }
            Storage::One(a) => {
                if index == 0 {
                    Storage::Two([value, a])
                } else {
                    Storage::Two([a, value])
                }
            }
            Storage::Two([a, b]) => match index {
                0 => Storage::Three([value, a, b]),
                1 => Storage::Three([a, value, b]),
                2 => Storage::Three([a, b, value]),
                _ => unreachable!()
            },
            Storage::Three([a, b, c]) => match index {
                0 => Storage::Four([value, a, b, c]),
                1 => Storage::Four([a, value, b, c]),
                2 => Storage::Four([a, b, value, c]),
                3 => Storage::Four([a, b, c, value]),
                _ => unreachable!()
            },
            Storage::Four([a, b, c, d]) => {
                let mut vec = Vec::with_capacity(INLINE_CAPACITY + 1);
                vec.extend([a, b, c, d]);
                vec.insert(index, value);
                Storage::Heap(vec)
            }
            Storage::Heap(mut vec) => {
                vec.insert(index, value);
                Storage::Heap(vec)
            }
        };
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            vec:   self,
            index: 0
        }
    }

    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns the elements as a mutable slice.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    /// Clears the vector, removing all elements.
    pub fn clear(&mut self) {
        self.storage = Storage::Empty;
    }

    /// Binary search by a key extracted from each element.
    ///
    /// Returns `Ok(index)` if found, `Err(index)` for insertion point.
    pub fn binary_search_by_key<'a, B, F>(&'a self, key: &B, mut f: F) -> Result<usize, usize>
    where
        B: Ord,
        F: FnMut(&'a T) -> B
    {
        self.as_slice().binary_search_by(|elem| f(elem).cmp(key))
    }

    /// Returns the elements as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> Default for InlineVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Manual Default impl to avoid requiring T: Default
#[allow(clippy::derivable_impls)]
impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> Deref for InlineVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match &self.storage {
            Storage::Empty => &[],
            Storage::One(a) => core::slice::from_ref(a),
            Storage::Two(arr) => arr.as_slice(),
            Storage::Three(arr) => arr.as_slice(),
            Storage::Four(arr) => arr.as_slice(),
            Storage::Heap(vec) => vec.as_slice()
        }
    }
}

impl<T> DerefMut for InlineVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.storage {
            Storage::Empty => &mut [],
            Storage::One(a) => core::slice::from_mut(a),
            Storage::Two(arr) => arr.as_mut_slice(),
            Storage::Three(arr) => arr.as_mut_slice(),
            Storage::Four(arr) => arr.as_mut_slice(),
            Storage::Heap(vec) => vec.as_mut_slice()
        }
    }
}

impl<T> FromIterator<T> for InlineVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}

impl<T> IntoIterator for InlineVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            storage: self.storage
        }
    }
}

impl<'a, T> IntoIterator for &'a InlineVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Borrowing iterator for [`InlineVec`].
#[derive(Debug)]
pub struct Iter<'a, T> {
    vec:   &'a InlineVec<T>,
    index: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

/// Owning iterator for [`InlineVec`].
#[derive(Debug)]
pub struct IntoIter<T> {
    storage: Storage<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.storage {
            Storage::Empty => None,
            Storage::One(_) => match core::mem::take(&mut self.storage) {
                Storage::One(a) => Some(a),
                _ => unreachable!()
            },
            Storage::Two(_) | Storage::Three(_) | Storage::Four(_) => {
                // Convert to heap storage for easier iteration
                let items: Vec<T> = match core::mem::take(&mut self.storage) {
                    Storage::Two([a, b]) => alloc::vec![a, b],
                    Storage::Three([a, b, c]) => alloc::vec![a, b, c],
                    Storage::Four([a, b, c, d]) => alloc::vec![a, b, c, d],
                    _ => unreachable!()
                };
                self.storage = Storage::Heap(items);
                self.next()
            }
            Storage::Heap(vec) => {
                if vec.is_empty() {
                    None
                } else {
                    Some(vec.remove(0))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let vec: InlineVec<i32> = InlineVec::new();
        assert!(vec.is_empty());
        assert!(vec.is_inline());
    }

    #[test]
    fn test_push_inline() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert!(vec.is_inline());
        assert_eq!(&*vec, &[1, 2, 3]);
    }

    #[test]
    fn test_push_to_capacity() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        assert_eq!(vec.len(), 4);
        assert!(vec.is_inline());
    }

    #[test]
    fn test_push_spill_to_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert!(!vec.is_inline());
        assert_eq!(&*vec, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_inline() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(3);
        vec.insert(1, 2);
        assert_eq!(&*vec, &[1, 2, 3]);
        assert!(vec.is_inline());
    }

    #[test]
    fn test_insert_at_start() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(2);
        vec.push(3);
        vec.insert(0, 1);
        assert_eq!(&*vec, &[1, 2, 3]);
    }

    #[test]
    fn test_insert_spill() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.insert(2, 99);
        assert!(!vec.is_inline());
        assert_eq!(&*vec, &[1, 2, 99, 3, 4]);
    }

    #[test]
    fn test_clone() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let cloned = vec.clone();
        assert_eq!(&*cloned, &*vec);
    }

    #[test]
    fn test_iter() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let collected: alloc::vec::Vec<_> = vec.iter().copied().collect();
        assert_eq!(collected, alloc::vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iter() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert_eq!(collected, alloc::vec![1, 2]);
    }

    #[test]
    fn test_clear() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_from_iter() {
        let vec: InlineVec<i32> = [1, 2, 3].into_iter().collect();
        assert_eq!(&*vec, &[1, 2, 3]);
    }

    #[test]
    fn test_deref() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
    }

    #[test]
    fn test_partial_eq() {
        let mut vec1: InlineVec<i32> = InlineVec::new();
        vec1.push(1);
        vec1.push(2);
        let mut vec2: InlineVec<i32> = InlineVec::new();
        vec2.push(1);
        vec2.push(2);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_with_strings() {
        let mut vec: InlineVec<alloc::string::String> = InlineVec::new();
        vec.push(alloc::string::String::from("hello"));
        vec.push(alloc::string::String::from("world"));
        assert_eq!(vec[0], "hello");
        assert_eq!(vec[1], "world");
    }

    #[test]
    fn test_get() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        if let Some(val) = vec.get_mut(0) {
            *val = 10;
        }
        assert_eq!(vec[0], 10);
    }

    #[test]
    fn test_get_three_elements() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));
        assert_eq!(vec.get(3), None);
    }

    #[test]
    fn test_get_four_elements() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(3), Some(&4));
        assert_eq!(vec.get(4), None);
    }

    #[test]
    fn test_get_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        assert!(!vec.is_inline());
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(5), Some(&6));
        assert_eq!(vec.get(6), None);
    }

    #[test]
    fn test_get_mut_three_elements() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        *vec.get_mut(2).unwrap() = 30;
        assert_eq!(vec[2], 30);
        assert!(vec.get_mut(3).is_none());
    }

    #[test]
    fn test_get_mut_four_elements() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        *vec.get_mut(3).unwrap() = 40;
        assert_eq!(vec[3], 40);
        assert!(vec.get_mut(4).is_none());
    }

    #[test]
    fn test_get_mut_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        *vec.get_mut(5).unwrap() = 60;
        assert_eq!(vec[5], 60);
        assert!(vec.get_mut(6).is_none());
    }

    #[test]
    fn test_insert_empty() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.insert(0, 42);
        assert_eq!(&*vec, &[42]);
    }

    #[test]
    fn test_insert_three_all_positions() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(3);
        vec.insert(1, 2);
        assert_eq!(&*vec, &[1, 2, 3]);

        let mut vec2: InlineVec<i32> = InlineVec::new();
        vec2.push(2);
        vec2.push(3);
        vec2.insert(0, 1);
        assert_eq!(&*vec2, &[1, 2, 3]);

        let mut vec3: InlineVec<i32> = InlineVec::new();
        vec3.push(1);
        vec3.push(2);
        vec3.insert(2, 3);
        assert_eq!(&*vec3, &[1, 2, 3]);
    }

    #[test]
    fn test_insert_four_all_positions() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.insert(0, 1);
        assert_eq!(&*vec, &[1, 2, 3, 4]);

        let mut vec2: InlineVec<i32> = InlineVec::new();
        vec2.push(1);
        vec2.push(3);
        vec2.push(4);
        vec2.insert(1, 2);
        assert_eq!(&*vec2, &[1, 2, 3, 4]);

        let mut vec3: InlineVec<i32> = InlineVec::new();
        vec3.push(1);
        vec3.push(2);
        vec3.push(4);
        vec3.insert(2, 3);
        assert_eq!(&*vec3, &[1, 2, 3, 4]);

        let mut vec4: InlineVec<i32> = InlineVec::new();
        vec4.push(1);
        vec4.push(2);
        vec4.push(3);
        vec4.insert(3, 4);
        assert_eq!(&*vec4, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_insert_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        vec.insert(3, 99);
        assert_eq!(vec[3], 99);
        assert_eq!(vec.len(), 7);
    }

    #[test]
    fn test_into_iter_three() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert_eq!(collected, alloc::vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iter_four() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert_eq!(collected, alloc::vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_into_iter_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert_eq!(collected, alloc::vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_into_iter_empty_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=5 {
            vec.push(i);
        }
        let mut iter = vec.into_iter();
        for _ in 0..5 {
            iter.next();
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_binary_search_by_key() {
        let mut vec: InlineVec<(i32, &str)> = InlineVec::new();
        vec.push((1, "a"));
        vec.push((3, "c"));
        vec.push((5, "e"));
        assert_eq!(vec.binary_search_by_key(&3, |&(k, _)| k), Ok(1));
        assert_eq!(vec.binary_search_by_key(&2, |&(k, _)| k), Err(1));
        assert_eq!(vec.binary_search_by_key(&6, |&(k, _)| k), Err(3));
    }

    #[test]
    fn test_iter_mut() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        for val in vec.iter_mut() {
            *val *= 10;
        }
        assert_eq!(&*vec, &[10, 20, 30]);
    }

    #[test]
    fn test_iter_size_hint() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut iter = vec.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let iter = vec.iter();
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn test_into_iter_for_ref() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let collected: alloc::vec::Vec<_> = (&vec).into_iter().copied().collect();
        assert_eq!(collected, alloc::vec![1, 2]);
    }

    #[test]
    fn test_default() {
        let vec: InlineVec<i32> = InlineVec::default();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_as_slice() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let slice = vec.as_slice();
        assert_eq!(slice, &[1, 2]);
    }

    #[test]
    fn test_as_mut_slice() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        vec.push(2);
        let slice = vec.as_mut_slice();
        slice[0] = 10;
        assert_eq!(vec[0], 10);
    }

    #[test]
    fn test_push_to_heap_then_more() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=10 {
            vec.push(i);
        }
        assert!(!vec.is_inline());
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn test_deref_empty() {
        let vec: InlineVec<i32> = InlineVec::new();
        let slice: &[i32] = &vec;
        assert!(slice.is_empty());
    }

    #[test]
    fn test_deref_mut_empty() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        let slice: &mut [i32] = &mut vec;
        assert!(slice.is_empty());
    }

    #[test]
    fn test_deref_one() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(42);
        let slice: &[i32] = &vec;
        assert_eq!(slice, &[42]);
    }

    #[test]
    fn test_deref_mut_one() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(42);
        let slice: &mut [i32] = &mut vec;
        slice[0] = 99;
        assert_eq!(vec[0], 99);
    }

    #[test]
    fn test_deref_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        let slice: &[i32] = &vec;
        assert_eq!(slice, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_deref_mut_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        let slice: &mut [i32] = &mut vec;
        slice[5] = 60;
        assert_eq!(vec[5], 60);
    }

    #[test]
    fn test_get_empty() {
        let vec: InlineVec<i32> = InlineVec::new();
        assert!(vec.get(0).is_none());
    }

    #[test]
    fn test_get_mut_empty() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        assert!(vec.get_mut(0).is_none());
    }

    #[test]
    fn test_get_one_out_of_bounds() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        assert!(vec.get(1).is_none());
    }

    #[test]
    fn test_get_mut_one_out_of_bounds() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(1);
        assert!(vec.get_mut(1).is_none());
    }

    #[test]
    fn test_len_heap() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        for i in 1..=10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn test_into_iter_one() {
        let mut vec: InlineVec<i32> = InlineVec::new();
        vec.push(42);
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert_eq!(collected, alloc::vec![42]);
    }

    #[test]
    fn test_into_iter_empty() {
        let vec: InlineVec<i32> = InlineVec::new();
        let collected: alloc::vec::Vec<_> = vec.into_iter().collect();
        assert!(collected.is_empty());
    }
}
