use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    const BASE_HUE: f32 = 275.0;

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let scale_point = scale.value();

        oklch(
            lerp(scale_point, 0.23..0.7),
            lerp(scale_point, 0.015..0.05),
            Self::BASE_HUE,
        )
    }

    pub(crate) fn fg(&self) -> Oklch {
        oklch(0.84, 0.03, Self::BASE_HUE)
    }

    pub(crate) fn bright_fg(&self) -> Oklch {
        oklch(0.93, 0.03, Self::BASE_HUE)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    FadedFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::FadedFg => 1.0,
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
