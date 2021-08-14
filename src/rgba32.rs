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

    pub fn to_rgb24(self) -> crate::Rgb24 {
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

    pub fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self {
        fn single_channel(channel: u8, numerator: u32, denominator: u32) -> u8 {
            let as_u32 = ((channel as u32) * (numerator)) / denominator;
            as_u32.min(::std::u8::MAX as u32) as u8
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
}
