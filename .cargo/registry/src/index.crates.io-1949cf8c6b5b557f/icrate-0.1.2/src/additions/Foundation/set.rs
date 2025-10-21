//! Utilities for the `NSSet` and `NSMutableSet` classes.
#![cfg(feature = "Foundation_NSSet")]
use alloc::vec::Vec;
use core::fmt;
use core::hash::Hash;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::mutability::{HasStableHash, IsRetainable};
use objc2::rc::IdFromIterator;

use super::iter;
use super::util;
use crate::common::*;
#[cfg(feature = "Foundation_NSMutableSet")]
use crate::Foundation::NSMutableSet;
use crate::Foundation::{self, NSSet};

impl<T: Message> NSSet<T> {
    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSSet::from_id_slice(&strs);
    /// assert_eq!(set.len(), 3);
    /// ```
    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSSet, NSString};
    ///
    /// let set = NSSet::<NSString>::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Message + Eq + Hash> NSSet<T> {
    /// Creates an [`NSSet`] from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
    /// let set = NSSet::from_vec(strs);
    /// ```
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self>
    where
        T: HasStableHash,
    {
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Creates an [`NSSet`] from a slice of `Id`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSSet::from_id_slice(&strs);
    /// ```
    pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
    where
        T: HasStableHash + IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::id_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Id<Self>
    where
        T: HasStableHash + IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Returns a [`Vec`] containing the set's elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableString, NSSet};
    ///
    /// let strs = vec![
    ///     NSMutableString::from_str("one"),
    ///     NSMutableString::from_str("two"),
    ///     NSMutableString::from_str("three"),
    /// ];
    /// let set = NSSet::from_vec(strs);
    /// let vec = set.to_vec();
    /// assert_eq!(vec.len(), 3);
    /// ```
    pub fn to_vec(&self) -> Vec<&T> {
        self.into_iter().collect()
    }

    #[doc(alias = "getObjects:range:")]
    pub fn to_vec_retained(&self) -> Vec<Id<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The objects are stored in the set
        self.into_iter()
            .map(|obj| unsafe { util::collection_retain_id(obj) })
            .collect()
    }

    /// Returns an [`NSArray`] containing the set's elements, or an empty
    /// array if the set is empty.
    ///
    /// [`NSArray`]: crate::Foundation::NSArray
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSNumber, NSSet, NSString};
    ///
    /// let nums = [1, 2, 3];
    /// let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));
    ///
    /// assert_eq!(set.to_array().len(), 3);
    /// assert!(set.to_array().iter().all(|i| nums.contains(&i.as_i32())));
    /// ```
    #[doc(alias = "allObjects")]
    #[cfg(feature = "Foundation_NSArray")]
    pub fn to_array(&self) -> Id<Foundation::NSArray<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The `T: IsIdCloneable` bound ensures that it is safe to
        // create what is effectively a copy of the collection from a `&self`
        // reference.
        //
        // Could be implemented as:
        //    NSArray::from_vec(self.to_vec_retained())
        unsafe { self.allObjects() }
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + Eq + Hash> NSMutableSet<T> {
    /// Creates an [`NSMutableSet`] from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
    /// let set = NSMutableSet::from_vec(strs);
    /// ```
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self>
    where
        T: HasStableHash,
    {
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Creates an [`NSMutableSet`] from a slice of `Id`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSMutableSet::from_id_slice(&strs);
    /// ```
    pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
    where
        T: HasStableHash + IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::id_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Id<Self>
    where
        T: HasStableHash + IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Returns a [`Vec`] containing the set's elements, consuming the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSMutableString};
    ///
    /// let strs = vec![
    ///     NSMutableString::from_str("one"),
    ///     NSMutableString::from_str("two"),
    ///     NSMutableString::from_str("three"),
    /// ];
    /// let set = NSMutableSet::from_vec(strs);
    /// let vec = NSMutableSet::into_vec(set);
    /// assert_eq!(vec.len(), 3);
    /// ```
    pub fn into_vec(set: Id<Self>) -> Vec<Id<T>> {
        set.into_iter().collect()
    }
}

extern_methods!(
    unsafe impl<T: Message> NSSet<T> {
        /// Returns a reference to one of the objects in the set, or `None` if
        /// the set is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// let any = set.get_any().unwrap();
        /// assert!(any == &*strs[0] || any == &*strs[1] || any == &*strs[2]);
        /// ```
        #[doc(alias = "anyObject")]
        #[method(anyObject)]
        pub fn get_any(&self) -> Option<&T>;
    }

    unsafe impl<T: Message + Eq + Hash> NSSet<T> {
        /// Returns `true` if the set contains a value.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{ns_string, NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// assert!(set.contains(ns_string!("one")));
        /// ```
        #[doc(alias = "containsObject:")]
        pub fn contains(&self, value: &T) -> bool {
            unsafe { self.containsObject(value) }
        }

        /// Returns a reference to the value in the set, if any, that is equal
        /// to the given value.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{ns_string, NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// assert_eq!(set.get(ns_string!("one")), Some(&*strs[0]));
        /// assert_eq!(set.get(ns_string!("four")), None);
        /// ```
        #[doc(alias = "member:")]
        #[method(member:)]
        pub fn get(&self, value: &T) -> Option<&T>;

        #[doc(alias = "member:")]
        pub fn get_retained(&self, value: &T) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            self.get(value)
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        // Note: No `get_mut` method exposed on sets, since their objects'
        // hashes are immutable.

        /// Returns `true` if the set is a subset of another, i.e., `other`
        /// contains at least all the values in `self`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(set1.is_subset(&set2));
        /// assert!(!set2.is_subset(&set1));
        /// ```
        #[doc(alias = "isSubsetOfSet:")]
        pub fn is_subset(&self, other: &NSSet<T>) -> bool {
            unsafe { self.isSubsetOfSet(other) }
        }

        /// Returns `true` if the set is a superset of another, i.e., `self`
        /// contains at least all the values in `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_superset(&set2));
        /// assert!(set2.is_superset(&set1));
        /// ```
        pub fn is_superset(&self, other: &NSSet<T>) -> bool {
            other.is_subset(self)
        }

        /// Returns `true` if `self` has no elements in common with `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        /// let set3 = NSSet::from_id_slice(&["four", "five", "six"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_disjoint(&set2));
        /// assert!(set1.is_disjoint(&set3));
        /// assert!(set2.is_disjoint(&set3));
        /// ```
        pub fn is_disjoint(&self, other: &NSSet<T>) -> bool {
            !unsafe { self.intersectsSet(other) }
        }
    }
);

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + Eq + Hash> NSMutableSet<T> {
    /// Add a value to the set. Returns whether the value was
    /// newly inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSNumber, NSMutableSet};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// assert_eq!(set.insert(&*NSNumber::new_u32(42)), true);
    /// assert_eq!(set.insert(&*NSNumber::new_u32(42)), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[doc(alias = "addObject:")]
    pub fn insert(&mut self, value: &T) -> bool
    where
        T: HasStableHash + IsRetainable,
    {
        let contains_value = self.contains(value);
        // SAFETY: Because of the `T: IsRetainable` bound, it is safe for the
        // set to retain the object here.
        unsafe { self.addObject(value) };
        !contains_value
    }

    /// Add an `Id` to the set. Returns whether the value was
    /// newly inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// assert_eq!(set.insert_id(NSString::from_str("one")), true);
    /// assert_eq!(set.insert_id(NSString::from_str("one")), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[doc(alias = "addObject:")]
    pub fn insert_id(&mut self, value: Id<T>) -> bool
    where
        T: HasStableHash,
    {
        let contains_value = self.contains(&value);
        // SAFETY: We've consumed ownership of the object.
        unsafe { self.addObject(&value) };
        !contains_value
    }

    /// Removes a value from the set. Returns whether the value was present
    /// in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{ns_string, NSMutableSet, NSString};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// set.insert_id(NSString::from_str("one"));
    /// assert_eq!(set.remove(ns_string!("one")), true);
    /// assert_eq!(set.remove(ns_string!("one")), false);
    /// ```
    #[doc(alias = "removeObject:")]
    pub fn remove(&mut self, value: &T) -> bool
    where
        T: HasStableHash,
    {
        let contains_value = self.contains(value);
        unsafe { self.removeObject(value) };
        contains_value
    }
}

// Iteration is not supposed to touch the elements, not even do comparisons.
//
// TODO: Verify that this is actually the case.
impl<T: Message> NSSet<T> {
    /// An iterator visiting all elements in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSSet::from_id_slice(&strs);
    /// for s in &set {
    ///     println!("{s}");
    /// }
    /// ```
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(super::iter::Iter::new(self))
    }

    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter_retained(&self) -> IterRetained<'_, T>
    where
        T: IsIdCloneable,
    {
        IterRetained(super::iter::IterRetained::new(self))
    }
}

unsafe impl<T: Message> iter::FastEnumerationHelper for NSSet<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSMutableSet<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of a `NSSet`.
#[derive(Debug)]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSSet<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for Iter<'a, T> { ... }
}

/// An iterator that retains the items of a `NSSet`.
#[derive(Debug)]
pub struct IterRetained<'a, T: Message>(iter::IterRetained<'a, NSSet<T>>);

__impl_iter! {
    impl<'a, T: Message + IsIdCloneable> Iterator<Item = Id<T>> for IterRetained<'a, T> { ... }
}

/// A consuming iterator over the items of a `NSSet`.
#[derive(Debug)]
pub struct IntoIter<T: Message>(iter::IntoIter<NSSet<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Id<T>> for IntoIter<T> { ... }
}

__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSSet<T> {
        type IntoIter = Iter<'_, T>;
    }

    #[cfg(feature = "Foundation_NSMutableSet")]
    impl<T: Message> IntoIterator for &NSMutableSet<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message + IsIdCloneable> IntoIterator for Id<NSSet<T>> {
        type IntoIter = IntoIter<T>;
    }

    #[cfg(feature = "Foundation_NSMutableSet")]
    impl<T: Message> IntoIterator for Id<NSMutableSet<T>> {
        type IntoIter = IntoIter<T>;
    }
}

impl<T: fmt::Debug + Message> fmt::Debug for NSSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + Eq + Hash + HasStableHash> Extend<Id<T>> for NSMutableSet<T> {
    fn extend<I: IntoIterator<Item = Id<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.insert_id(item);
        })
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<'a, T: Message + Eq + Hash + HasStableHash + IsRetainable> Extend<&'a T> for NSMutableSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.insert(item);
        })
    }
}

impl<'a, T: Message + Eq + Hash + HasStableHash + IsRetainable + 'a> IdFromIterator<&'a T>
    for NSSet<T>
{
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message + Eq + Hash + HasStableHash> IdFromIterator<Id<T>> for NSSet<T> {
    fn id_from_iter<I: IntoIterator<Item = Id<T>>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<'a, T: Message + Eq + Hash + HasStableHash + IsRetainable + 'a> IdFromIterator<&'a T>
    for NSMutableSet<T>
{
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + Eq + Hash + HasStableHash> IdFromIterator<Id<T>> for NSMutableSet<T> {
    fn id_from_iter<I: IntoIterator<Item = Id<T>>>(iter: I) -> Id<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}
