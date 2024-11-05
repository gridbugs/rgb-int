#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct WeightsU16 {
    r: u16,
    g: u16,
    b: u16,
    sum: u32,
}

impl WeightsU16 {
    pub const fn new(r: u16, g: u16, b: u16) -> Self {
        Self {
            r,
            g,
            b,
            sum: r as u32 + g as u32 + b as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Rgb24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb24 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn hex(hex: u32) -> Self {
        let [b, g, r, _] = hex.to_le_bytes();
        Self { r, g, b }
    }

    pub const fn with_r(self, r: u8) -> Self {
        Self { r, ..self }
    }

    pub const fn with_g(self, g: u8) -> Self {
        Self { g, ..self }
    }

    pub const fn with_b(self, b: u8) -> Self {
        Self { b, ..self }
    }
    pub const fn new_grey(c: u8) -> Self {
        Self::new(c, c, c)
    }
    pub fn floor(self, min: u8) -> Self {
        Self {
            r: self.r.max(min),
            g: self.g.max(min),
            b: self.b.max(min),
        }
    }
    pub fn ceil(self, max: u8) -> Self {
        Self {
            r: self.r.min(max),
            g: self.g.min(max),
            b: self.b.min(max),
        }
    }
    pub const fn to_rgba32(self, a: u8) -> crate::Rgba32 {
        let Self { r, g, b } = self;
        crate::Rgba32 { r, g, b, a }
    }
    pub fn to_f32_array_01(self) -> [f32; 3] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        ]
    }
    pub fn to_f32_array_01_rgba(self, alpha: f32) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            alpha,
        ]
    }
    pub const fn saturating_add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
        }
    }
    pub const fn saturating_scalar_mul(self, scalar: u32) -> Self {
        const fn single_channel(channel: u8, scalar: u32) -> u8 {
            let as_u32 = channel as u32 * scalar;
            if as_u32 > ::std::u8::MAX as u32 {
                ::std::u8::MAX
            } else {
                as_u32 as u8
            }
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub fn saturating_scalar_mul_f64(self, scalar: f64) -> Self {
        fn single_channel(channel: u8, scalar: f64) -> u8 {
            let as_f64 = channel as f64 * scalar;
            if as_f64 > ::std::u8::MAX as f64 {
                ::std::u8::MAX
            } else {
                as_f64 as u8
            }
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub const fn scalar_div(self, scalar: u32) -> Self {
        const fn single_channel(channel: u8, scalar: u32) -> u8 {
            let as_u32 = channel as u32 / scalar;
            if as_u32 > ::std::u8::MAX as u32 {
                ::std::u8::MAX
            } else {
                as_u32 as u8
            }
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub const fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self {
        const fn single_channel(channel: u8, numerator: u32, denominator: u32) -> u8 {
            let as_u32 = ((channel as u32) * (numerator)) / denominator;
            if as_u32 > ::std::u8::MAX as u32 {
                ::std::u8::MAX
            } else {
                as_u32 as u8
            }
        }
        Self {
            r: single_channel(self.r, numerator, denominator),
            g: single_channel(self.g, numerator, denominator),
            b: single_channel(self.b, numerator, denominator),
        }
    }
    pub const fn normalised_mul(self, other: Self) -> Self {
        const fn single_channel(a: u8, b: u8) -> u8 {
            ((a as u32 * b as u32) / 255) as u8
        }
        Self {
            r: single_channel(self.r, other.r),
            g: single_channel(self.g, other.g),
            b: single_channel(self.b, other.b),
        }
    }
    pub const fn normalised_scalar_mul(self, scalar: u8) -> Self {
        const fn single_channel(c: u8, scalar: u8) -> u8 {
            ((c as u32 * scalar as u32) / 255) as u8
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub const fn linear_interpolate(self, to: Rgb24, by: u8) -> Self {
        const fn interpolate_channel(from: u8, to: u8, by: u8) -> u8 {
            let total_delta = to as i32 - from as i32;
            let current_delta = (total_delta * by as i32) / 255;
            (from as i32 + current_delta) as u8
        }
        Self {
            r: interpolate_channel(self.r, to.r, by),
            g: interpolate_channel(self.g, to.g, by),
            b: interpolate_channel(self.b, to.b, by),
        }
    }
    pub fn min_channel(self) -> u8 {
        self.r.min(self.g).min(self.b)
    }
    pub fn max_channel(self) -> u8 {
        self.r.max(self.g).max(self.b)
    }
    pub fn saturating_channel_total(self) -> u8 {
        self.r.saturating_add(self.g).saturating_add(self.b)
    }
    pub const fn complement(self) -> Self {
        Self {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }
    pub const fn weighted_mean_u16(self, weights: WeightsU16) -> u8 {
        let weighted_sum = self.r as u32 * weights.r as u32
            + self.g as u32 * weights.g as u32
            + self.b as u32 * weights.b as u32;
        (weighted_sum / weights.sum) as u8
    }
}

#[cfg(feature = "rand")]
mod sample {
    use super::Rgb24;
    use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
    use rand::prelude::*;

    pub struct UniformRgb24LinearInterpolate {
        inner: UniformInt<u8>,
        low: Rgb24,
        high: Rgb24,
    }

    impl UniformSampler for UniformRgb24LinearInterpolate {
        type X = Rgb24;
        fn new<B1, B2>(low: B1, high: B2) -> Self
        where
            B1: SampleBorrow<Self::X> + Sized,
            B2: SampleBorrow<Self::X> + Sized,
        {
            Self {
                inner: UniformInt::<u8>::new(::std::u8::MIN, ::std::u8::MAX),
                low: *low.borrow(),
                high: *high.borrow(),
            }
        }
        fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
        where
            B1: SampleBorrow<Self::X> + Sized,
            B2: SampleBorrow<Self::X> + Sized,
        {
            UniformSampler::new(low, high)
        }
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
            self.low
                .linear_interpolate(self.high, self.inner.sample(rng))
        }
    }

    impl SampleUniform for Rgb24 {
        type Sampler = UniformRgb24LinearInterpolate;
    }
}

#[cfg(feature = "rand")]
pub use sample::UniformRgb24LinearInterpolate;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = Rgb24::new(255, 0, 200);
        let b = Rgb24::new(0, 255, 200);
        let c = a.saturating_add(b);
        assert_eq!(c, Rgb24::new(255, 255, 255));
    }

    #[test]
    fn sub() {
        let a = Rgb24::new(255, 0, 200);
        let b = Rgb24::new(0, 255, 200);
        let c = a.saturating_sub(b);
        assert_eq!(c, Rgb24::new(255, 0, 0));
    }

    #[test]
    fn mul_div() {
        assert_eq!(
            Rgb24::new(1, 2, 3).saturating_scalar_mul_div(1500, 1000),
            Rgb24::new(1, 3, 4)
        );
        assert_eq!(
            Rgb24::new(1, 2, 3).saturating_scalar_mul_div(1500, 1),
            Rgb24::new(255, 255, 255)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Rgb24::new(20, 40, 60).saturating_scalar_mul(2),
            Rgb24::new(40, 80, 120),
        );
        assert_eq!(
            Rgb24::new(20, 40, 60).saturating_scalar_mul(10000),
            Rgb24::new(255, 255, 255),
        );
    }

    #[test]
    fn div() {
        assert_eq!(Rgb24::new(20, 40, 60).scalar_div(2), Rgb24::new(10, 20, 30));
        assert_eq!(
            Rgb24::new(255, 255, 255).scalar_div(256),
            Rgb24::new(0, 0, 0)
        );
    }

    #[test]
    #[should_panic]
    fn div_by_zero() {
        Rgb24::new(0, 0, 0).scalar_div(0);
    }

    #[test]
    fn normalised_mul() {
        assert_eq!(
            Rgb24::new(255, 255, 255).normalised_mul(Rgb24::new(1, 2, 3)),
            Rgb24::new(1, 2, 3)
        );
        assert_eq!(
            Rgb24::new(255, 127, 0).normalised_mul(Rgb24::new(10, 20, 30)),
            Rgb24::new(10, 9, 0)
        );
    }

    #[test]
    fn grey() {
        assert_eq!(Rgb24::new_grey(37), Rgb24::new(37, 37, 37));
    }

    #[test]
    fn floor() {
        assert_eq!(Rgb24::new(100, 5, 0).floor(10), Rgb24::new(100, 10, 10));
    }

    #[test]
    fn ceil() {
        assert_eq!(Rgb24::new(255, 250, 20).ceil(200), Rgb24::new(200, 200, 20));
    }

    #[test]
    fn normalised_scalar_mul() {
        assert_eq!(
            Rgb24::new(255, 128, 0).normalised_scalar_mul(128),
            Rgb24::new(128, 64, 0)
        );
    }

    #[test]
    fn interpolate() {
        let from = Rgb24::new(0, 255, 100);
        let to = Rgb24::new(255, 0, 120);
        assert_eq!(from.linear_interpolate(to, 0), from);
        assert_eq!(from.linear_interpolate(to, 255), to);
        assert_eq!(from.linear_interpolate(to, 63), Rgb24::new(63, 192, 104));
    }

    #[test]
    fn weighted_mean() {
        assert_eq!(
            Rgb24::new(14, 120, 201).weighted_mean_u16(WeightsU16::new(0, 0, 1)),
            201
        );
        assert_eq!(
            Rgb24::new(14, 120, 201).weighted_mean_u16(WeightsU16::new(299, 587, 114)),
            97
        );
        assert_eq!(
            Rgb24::new(0, 0, 0).weighted_mean_u16(WeightsU16::new(299, 587, 114)),
            0
        );
        assert_eq!(
            Rgb24::new(255, 255, 255).weighted_mean_u16(WeightsU16::new(299, 587, 114)),
            255
        );
        assert_eq!(
            Rgb24::new(255, 255, 255).weighted_mean_u16(WeightsU16::new(
                std::u16::MAX,
                std::u16::MAX,
                std::u16::MAX
            )),
            255
        );
    }

    #[test]
    #[should_panic]
    fn weighted_mean_zero() {
        Rgb24::new(1, 2, 3).weighted_mean_u16(WeightsU16::new(0, 0, 0));
    }

    #[test]
    fn hex() {
        assert_eq!(Rgb24::hex(0x123456), Rgb24::new(0x12, 0x34, 0x56));
    }
}
