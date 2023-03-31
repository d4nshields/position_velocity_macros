# Position and Velocity Macros

This Rust library provides custom derive macros for implementing `Position` and `Velocity` traits for user-defined types.

## Usage

Add this library to your `Cargo.toml`:

```toml
[dependencies]
position_velocity_macros = "0.1.0"
```

Then, import the macros in your code:

```rust
use position_velocity_macros::{Position, Velocity};
```

## Position

The Position trait represents an object with a position. You can derive this trait for your custom type, and it will automatically implement the position method.

```rust
use position_velocity_macros::Position;

#[derive(Position)]
struct Particle<T> {
    position: T,
}
```

## Velocity

The Velocity trait represents an object with both position and velocity. Deriving this trait for a type will automatically implement the Velocity trait, as well as the Position trait.

```rust
use position_velocity_macros::Velocity;

#[derive(Velocity)]
struct MovingParticle<T> {
    position: T,
    velocity: T,
}
```

## Example

Here's a complete example using both `Position` and `Velocity` derived traits, along with the `MulF64` and `AddF64` traits:

```rust
use position_velocity_macros::{Position, Velocity};
use std::ops::Mul;

pub trait AddF64 {
    fn add_f64(&self, other: &Self) -> Self;
}

pub trait MulF64 {
    fn mul_f64(&self, f: f64) -> Self;
}

pub trait Position {
    type T;
    fn position(&self) -> &Self::T;
}

pub trait Velocity: Position
where
    Self::T: AddF64 + MulF64,
{
    fn velocity(&self) -> &Self::T;
    fn update_position(&self, t: f64) -> Self::T {
        self.position().add_f64(&self.velocity().mul_f64(t))
    }
}

#[derive(Position)]
struct Particle<T> {
    position: T,
}

#[derive(Velocity)]
struct MovingParticle<T: MulF64 + AddF64> {
    position: T,
    velocity: T,
}

// Implement AddF64 for the tuple (f64, f64)
impl AddF64 for (f64, f64) {
    fn add_f64(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
}

// Implement MulF64 for the tuple (f64, f64)
impl MulF64 for (f64, f64) {
    fn mul_f64(&self, f: f64) -> Self {
        (self.0 * f, self.1 * f)
    }
}

```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

