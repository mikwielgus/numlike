// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A wrapper to turn this library's trait methods into inherent methods.

/// A wrapper to turn this library's trait methods into inherent methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object<B>(pub(crate) B);

impl<B> Object<B> {
    /// Wrap the backend inside an object type.
    pub fn new(backend: B) -> Self {
        Self(backend)
    }

    /// Unwrap the backend out of the object wrapper type.
    pub fn dissolve(self) -> B {
        self.0
    }
}
