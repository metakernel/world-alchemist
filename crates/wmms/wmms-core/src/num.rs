use crate::error::{Result, WMMSCoreError};

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct FixedU32<const FRAC_BITS: u32>(pub i32);

pub type Q16_16 = FixedU32<16>;
pub type Q24_8 = FixedU32<8>;

impl<const FRAC: u32> FixedU32<FRAC> {
    pub const SCALE:i64 = 1i64 << FRAC;

    #[inline]
    pub fn from_i32(v: i32)->Self{
        Self(v.saturating_mul(Self::SCALE as i32))
    }

    #[inline]
    pub fn to_f32(self)->f32{
        (self.0 as f32) / (Self::SCALE as f32)
    }

    // Quantized conversion from f32 to FixedU32
    // Rounds to nearest representable value
    pub fn from_f32_quantized(v: f32) -> Self{
        let scaled = (v as f64) * (Self::SCALE as f64);
        let raw = if scaled >= 0.0 {
            (scaled + 0.5).floor() as i32
        } else {
            (scaled - 0.5).ceil() as i32
        };

        Self(raw as i32)
    }

    pub fn add(self, other:Self) -> Self{
        Self(self.0.saturating_add(other.0))
    }
    pub fn sub(self, other:Self) -> Self{
        Self(self.0.saturating_sub(other.0))
    }

    pub fn mul(self, other:Self) -> Self{
        let a = self.0 as i64;
        let b = other.0 as i64;
        let raw = (a *b) >> FRAC;
        Self(raw as i32)
    }

    pub fn div (self, other: Self) -> Result<Self> {
        if other.0 == 0 {
            return Err(WMMSCoreError::InvalidValue("Division by zero".into()));
        }
        let a =(self.0 as i64) << FRAC;
        let b = other.0 as i64;
        let raw = a / b;
        Ok(Self(raw.clamp(i32::MIN as i64, i32::MAX as i64) as i32))
    }
}

// Quantization helper
pub fn quantize_f32(v: f32, step: f32) -> f32 {
    if step == 0.0 {
        return v;
    }
    let q = (v / step) as f64;
    let r = if q >= 0.0 {
        (q + 0.5).floor()
    } else {
        (q - 0.5).ceil()
    };

    (r as f32) * step
}

