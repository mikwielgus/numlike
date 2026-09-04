// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/undoredo")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, doc = "\n## Feature flags\n")]
#![cfg_attr(docsrs, doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod bytes;
pub mod convert;
pub mod elem;
pub mod limits;
//pub mod group;
pub mod cmp;
pub mod fns;
pub mod ops;
//pub mod ring;
