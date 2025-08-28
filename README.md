# Isotopes

> This is currently in early development. None of the API is stable.

Variants of common types for enforcing invariance.

## Types

### Non-negative

Types that cannot go under `0`. They implement `Add<Self>`, `Mul<Self>`, and `Div<Self>` that just do exactly what their underlying types do. `Sub` was not implemented, since underflowing is extremely easy to accidentally do.

- `NonNegativeU8` - `NonNegativeU128`, `NonNegativeUsize`
- `NonNegativeI8` - `NonNegativeI128`, `NonNegativeIsize`
- `NonNegativeF32`, `NonNegativeF64`

### Clamped ratio

A ratio that is between `0` and `1`. They use `f32`/`f64` under the hood. They implement the [checked operator](#checked-operator-traits) and [bound](#bound-traits) traits.

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

Each of these traits provide one function (e.g., `checked_add`) that lets you add two numbers and returns a `Result`. You can customize the `Error` type.

### Saturating type

`Saturating` is a wrapper struct that automatically implements the standard operator traits `Add`, `Sub`, `Mul`, and `Div` for all implementations of the `Checked*` traits that also implement `Min` and `Max`. Underflowing and overflowing the underlying type just returns its min and max values respectively.
