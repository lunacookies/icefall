use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    const BASE_HUE: f32 = 275.0;

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), Self::BASE_HUE)
    }

    const COLOR_LIGHTNESS: f32 = 0.7;

    pub(crate) fn red(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS + 0.05, 0.1, 30.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS + 0.05, 0.1, 55.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS, 0.07, 115.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS, 0.05, 215.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS, 0.07, 255.0)
    }

    pub(crate) fn blue_2(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS + 0.1, 0.05, 255.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(Self::COLOR_LIGHTNESS + 0.05, 0.075, 295.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkBg,
    Bg,
    MiddleBg,
    LightBg,
    BarelyVisibleFg,
    DarkFg,
    DimFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::DarkBg => 0.0,
            Self::Bg => 0.05,
            Self::MiddleBg => 0.08,
            Self::LightBg => 0.25,
            Self::BarelyVisibleFg => 0.3,
            Self::DarkFg => 0.5,
            Self::DimFg => 0.7,
            Self::Fg => 0.8,
            Self::BrightFg => 1.0,
        }
    }

    fn lightness(self) -> f32 {
        lerp(self.value(), 0.2..0.98)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::DarkBg | Self::Bg => 0.02,
            Self::Fg | Self::BrightFg => 0.01,
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
