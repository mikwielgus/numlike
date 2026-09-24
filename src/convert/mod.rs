// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Conversions from one type to another, possibly lossy.

mod cast;
mod quantize;

pub use cast::*;
pub use quantize::*;
