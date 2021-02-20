use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    const BASE_HUE: f32 = 277.0;

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), Self::BASE_HUE)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    LightenedBg,
    BarelyVisibleFg,
    DimmedFg,
    FadedFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::DarkBg => 0.0,
            Self::Bg => 0.05,
            Self::LightenedBg => 0.1,
            Self::BarelyVisibleFg => 0.3,
            Self::DimmedFg => 0.5,
            Self::FadedFg => 0.7,
            Self::Fg => 0.9,
            Self::BrightFg => 1.0,
        }
    }

    fn lightness(self) -> f32 {
        lerp(self.value(), 0.2..0.93)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::DarkBg | Self::Bg => 0.02,
            Self::Fg | Self::BrightFg => 0.03,
            _ => lerp(self.value(), 0.03..0.08),
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
