<!--
SPDX-FileCopyrightText: 2026 numlike contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Repository](https://img.shields.io/badge/repository-GitHub-0FBF3E)](https://github.com/mikwielgus/numlike)
[![Docs](https://docs.rs/numlike/badge.svg)](https://docs.rs/numlike/)
[![Crates.io](https://img.shields.io/crates/v/numlike.svg)](https://crates.io/crates/numlike)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/numlike.svg)](#licence)

# numlike

Numeric traits for generic mathematics. Less restrictive and less conservative
alternative to [`num-traits`](https://docs.rs/num-traits/latest/num_traits/).

This crate has no `unsafe` code and no mandatory third-party dependencies, and
is `no_std`-compatible. Most of `no_std` operations on floating point numbers
still have a dependency on [`libm`](https://docs.rs/libm/latest/libm/), which is
gated behind `libm` feature flag.

## Usage

### Adding dependency

```toml
[dependencies]
numlike = { version = "0.1.3" }
```

## Comparison to other libraries

### Why `numlike` and not `num-traits`?

We developed `numlike` primarily because we disagree with many of design
decisions in the venerable `num-traits` crate.

- `num-traits`'s
  [`Zero`](https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html)
  and
  [`One`](https://docs.rs/num-traits/latest/num_traits/identities/trait.One.html)
  traits require `Add` and `Mul` traits, respectively, to be
  implemented. This makes it impossible to distinguish a *0* for
  algebraic structures that don't implement addition (e.g. [absorption
  magma](https://ncatlab.org/nlab/show/absorption+magma) and [absorption
  monoid](https://ncatlab.org/nlab/show/absorption+monoid), aka.
  *magma with zero* and *monoid with zero*, where *0* is merely the
  [absorbing element](https://en.wikipedia.org/wiki/Absorbing_element)),
  and likewise *1* when there is no multiplication (e.g. because a
  naive implementation of multiplication for all elements would be
  inefficient, or because *1* is merely the generating element (aka.
  [generator](https://en.wikipedia.org/wiki/Generator_(mathematics)))).
  - `num-traits` also requires `Output = Self` for `Add` and `Mul`, making it
    impossible to use `Zero` and `One` for statically-typed unit of measurement
    libraries like [`uom`](https://docs.rs/uom/latest/uom/).
    - `numlike` does not have these problems because it does not have any
      supertraits for its `Zero` and `One`.
  - Moreover, `num-traits`'s `Zero` and `One` do not provide `ZERO` and `ONE`
    associated constants, but instead return them from `::zero()` and `::one()`
    functions. These constants were only later added through new separate
    traits, `ConstZero` and `ConstOne`, presumably to avoid breaking changes.
    - `numlike`'s `Zero` and `One` provide `ZERO` and `ONE` associated
      constants directly.
- `num-traits`'s
  [`Bounded`](https://docs.rs/num-traits/latest/num_traits/bounds/trait.Bounded.html)
  trait only returns **finite** minimum and maximum values. This makes no
  difference for integers, but e.g. for floats `.max_value()` returns
  [`f32::MAX`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX),
  which is actually the largest finite number, equal to `3.40282347e+38`, not
  the positive infinity. `num-traits` has no interface to generically obtain
  negative or positive infinity as min. or max. value.
  - `numlike` solves that by providing
    [`MinExtended`](https://docs.rs/numlike/latest/numlike/limits/trait.MinExtended.html)/
    [`MaxExtended`](https://docs.rs/numlike/latest/numlike/limits/trait.MaxExtended.html)
    traits that result in negative and positive infinities for floats, and
    [`MinFinite`](https://docs.rs/numlike/latest/numlike/limits/trait.MinFinite.html)/
    [`MaxFinite`](https://docs.rs/numlike/latest/numlike/limits/trait.MaxFinite.html)
    traits that give only finite values just as above `num-traits`'s `Bounded`
    does.
- `num-traits` provides `.signum()` and
  `.abs()` methods only for types implementing
  [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Signed.html)
  trait, which excludes unsigned integer types.
  - But having these methods generically for both signed and unsigned types
    can be useful for finding canonical denominators, reducing fractions,
    combining and simplifying radicals, so `numlike` provides these methods
    through two decoupled traits,
    [`Signum`](https://docs.rs/numlike/latest/numlike/ops/trait.Signum.html)
    and [`Abs`](https://docs.rs/numlike/latest/numlike/ops/trait.Abs.html),
    implemented for all numeric primitives.
- `num-traits` does not provide checked mathematical operation traits,
  [`CheckedAdd`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedAdd.html),
  [`CheckedSub`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedSub.html),
  [`CheckedMul`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedMul.html),
  [`CheckedDiv`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedDiv.html),
  [`CheckedNeg`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedNeg.html),
  [`CheckedRem`](https://docs.rs/num-traits/latest/num_traits/ops/checked/trait.CheckedRem.html),
  for floats.
  - `numlike` implements its own versions of these traits for all numeric
    primitives.
- Developers of `num-traits` seem to avoid breaking changes, strongly preferring
  to create new traits instead of making modifications to existing ones. This
  provides stability for users, but prevents at least some of the above issues
  from being solved.
  - Because of that, we have decided to roll our own library (this crate).
    However, because it's in active development, we are lacking the stability of
    `num-traits` -- we are much more likely to have breaking changes and bugs.
- Furthermore, `numlike` also has its own features, such as:
  - Equality and order traits that fix `NaN`s to be the highest value in the
    set, even larger than positive infinity, allowing for total order:
    [`NanPartialEq`](https://docs.rs/numlike/latest/numlike/cmp/trait.NanPartialEq.html),
    [`NanEq`](https://docs.rs/numlike/latest/numlike/cmp/trait.NanEq.html),
    [`NanPartialOrd`](https://docs.rs/numlike/latest/numlike/cmp/trait.NanPartialOrd.html),
    [`NanOrd`](https://docs.rs/numlike/latest/numlike/cmp/trait.NanOrd.html).
    - But if you want `core::cmp` traits instead, consider using `ordered-float`
      crate's
      [`OrderedFloat`](https://docs.rs/ordered-float/latest/ordered_float/struct.OrderedFloat.html)
      float type wrapper instead. This is obviously often the better choice,
      because `core::cmp` traits will work with a much larger number of existing
      libraries and have Rust's syntactic sugar.
