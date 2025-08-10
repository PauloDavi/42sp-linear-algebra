pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}

pub trait Negative {
    fn negative_one() -> Self;
}

pub trait Magnitude {
    type Output;

    fn magnitude(&self) -> Self::Output;
}

pub trait Conjugate {
    fn conjugate(&self) -> Self;
}

impl Zero for i8 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i8 {
    fn one() -> Self {
        1
    }
}

impl Negative for i8 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i16 {
    fn one() -> Self {
        1
    }
}

impl Negative for i16 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl Negative for i32 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i64 {
    fn one() -> Self {
        1
    }
}

impl Negative for i64 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

impl Zero for u16 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u16 {
    fn one() -> Self {
        1
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u64 {
    fn one() -> Self {
        1
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl Negative for f32 {
    fn negative_one() -> Self {
        -1.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl Negative for f64 {
    fn negative_one() -> Self {
        -1.0
    }
}

impl Magnitude for i8 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i16 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for u8 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u16 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for f32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        self.abs()
    }
}

impl Magnitude for f64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Conjugate for i8 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i16 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i64 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u8 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u16 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u64 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for f32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for f64 {
    fn conjugate(&self) -> Self {
        *self
    }
}
