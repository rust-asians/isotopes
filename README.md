# Isotopes

> This is currently in early development. None of the API is stable.

Variants of common types for enforcing invariance.

## Types

### Unsigned

Types that cannot go under `0`. They implement `Add<Self>`, `Mul<Self>`, and `Div<Self>` that just do exactly what their
underlying types do. `Sub` was not implemented, since underflowing is extremely easy to accidentally do.

The lack of `Sub` is still under consideration, since unsigned integers _do_ implement it. However, it's still unclear
if we should follow this unsafe implementation out of consistency, or if it's worth having a wrapper around unsigned
integers that does hides `Sub`.

They implement the [checked operator](#checked-operator-traits) and [bound](#bound-traits) traits.

- `UF32`
- `UF64`

### Clamped ratio

A ratio that is between `0` and `1`. They use `f32`/`f64` under the hood. They implement
the [checked operator](#checked-operator-traits) and [bound](#bound-traits) traits.

- `ClampedRatio32`
- `ClampedRatio64`

## Operators

### Bound traits

- Min
- Max

These just provide a standard way of defining the upper and lower limits of a type.

Implementations of Min and Max for primitive ints (signed and unsigned) and floats are already defined.

### Checked operator traits

- `CheckedAdd`
- `CheckedSub`
- `CheckedMul`
- `CheckedDiv`

Each of these traits provide one function (e.g., `checked_add`) that lets you add two numbers and returns a `Result`.
You can customize the `Error` type.

### Saturating type

`Saturating` is a wrapper struct that automatically implements the standard operator traits `Add`, `Sub`, `Mul`, and
`Div` for all implementations of the `Checked*` traits that also implement `Min` and `Max`. Underflowing and overflowing
the underlying type just returns its min and max values respectively.
