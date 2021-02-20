use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    const BASE_HUE: f32 = 277.0;

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), Self::BASE_HUE)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.75, 0.05, 215.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.75, 0.1, 290.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    MiddleBg,
    LightBg,
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
            Self::MiddleBg => 0.1,
            Self::LightBg => 0.25,
            Self::BarelyVisibleFg => 0.3,
            Self::DimmedFg => 0.5,
            Self::FadedFg => 0.7,
            Self::Fg => 0.8,
            Self::BrightFg => 1.0,
        }
    }

    fn lightness(self) -> f32 {
        lerp(self.value(), 0.2..0.96)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::DarkBg | Self::Bg => 0.02,
            Self::Fg => 0.03,
            Self::BrightFg => 0.02,
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
