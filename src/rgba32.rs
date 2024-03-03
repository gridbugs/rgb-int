#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Rgba32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba32 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn new_grey(x: u8) -> Self {
        Self {
            r: x,
            g: x,
            b: x,
            a: 255,
        }
    }

    pub const fn hex(hex: u32) -> Self {
        let [a, b, g, r] = hex.to_le_bytes();
        Self { r, g, b, a }
    }

    pub const fn hex_rgb(hex: u32) -> Self {
        let [b, g, r, _] = hex.to_le_bytes();
        Self { r, g, b, a: 255 }
    }

    pub const fn to_rgb24(self) -> crate::Rgb24 {
        let Self { r, g, b, a: _ } = self;
        crate::Rgb24 { r, g, b }
    }

    pub fn to_f32_array_01(self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }

    pub fn to_f32_array_rgb_01(self) -> [f32; 3] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        ]
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

    pub const fn with_a(self, a: u8) -> Self {
        Self { a, ..self }
    }

    pub const fn linear_interpolate(self, to: Rgba32, by: u8) -> Self {
        const fn interpolate_channel(from: u8, to: u8, by: u8) -> u8 {
            let total_delta = to as i32 - from as i32;
            let current_delta = (total_delta * by as i32) / 255;
            (from as i32 + current_delta) as u8
        }
        Self {
            r: interpolate_channel(self.r, to.r, by),
            g: interpolate_channel(self.g, to.g, by),
            b: interpolate_channel(self.b, to.b, by),
            a: interpolate_channel(self.a, to.a, by),
        }
    }

    pub fn alpha_composite(self, below: Rgba32) -> Rgba32 {
        fn mul_u8(a: u8, b: u8) -> u8 {
            ((a as u16 * b as u16) / 255) as u8
        }
        fn div_u8(a: u8, b: u8) -> u8 {
            ((255 * a as u16) / b as u16) as u8
        }
        let alpha_out_rhs = mul_u8(below.a, 255 - self.a);
        let alpha_out = self.a + alpha_out_rhs;
        let single_channel =
            |c_a: u8, c_b: u8| div_u8(mul_u8(c_a, self.a) + mul_u8(c_b, alpha_out_rhs), alpha_out);
        Self {
            r: single_channel(self.r, below.r),
            g: single_channel(self.g, below.g),
            b: single_channel(self.b, below.b),
            a: alpha_out,
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
            a: self.a,
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
            a: self.a,
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
            a: self.a,
        }
    }
}

#[cfg(feature = "rand")]
mod sample {
    use super::Rgba32;
    use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
    use rand::prelude::*;

    pub struct UniformRgba32LinearInterpolate {
        inner: UniformInt<u8>,
        low: Rgba32,
        high: Rgba32,
    }

    impl UniformSampler for UniformRgba32LinearInterpolate {
        type X = Rgba32;
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

    impl SampleUniform for Rgba32 {
        type Sampler = UniformRgba32LinearInterpolate;
    }
}

#[cfg(feature = "rand")]
pub use sample::UniformRgba32LinearInterpolate;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalised_mul() {
        assert_eq!(
            Rgba32::new(255, 255, 255, 127).normalised_mul(Rgba32::new(1, 2, 3, 127)),
            Rgba32::new(1, 2, 3, 127)
        );
        assert_eq!(
            Rgba32::new(255, 127, 0, 127).normalised_mul(Rgba32::new(10, 20, 30, 127)),
            Rgba32::new(10, 9, 0, 127)
        );
    }

    #[test]
    fn normalised_scalar_mul() {
        assert_eq!(
            Rgba32::new(255, 128, 0, 127).normalised_scalar_mul(128),
            Rgba32::new(128, 64, 0, 127)
        );
    }

    #[test]
    fn interpolate() {
        let from = Rgba32::new(0, 255, 100, 127);
        let to = Rgba32::new(255, 0, 120, 127);
        assert_eq!(from.linear_interpolate(to, 0), from);
        assert_eq!(from.linear_interpolate(to, 255), to);
        assert_eq!(
            from.linear_interpolate(to, 63),
            Rgba32::new(63, 192, 104, 127)
        );
    }

    #[test]
    fn hex() {
        assert_eq!(Rgba32::hex(0x12345678), Rgba32::new(0x12, 0x34, 0x56, 0x78));
    }

    #[test]
    fn hex_rgb() {
        assert_eq!(Rgba32::hex_rgb(0x123456), Rgba32::new_rgb(0x12, 0x34, 0x56));
    }
}
