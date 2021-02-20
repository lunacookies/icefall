mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut icefall = ThemeBuilder::new("Icefall".to_string(), Type::Dark);
    imp::add_rules(&mut icefall, &palette);
    icefall.build().save()?;

    Ok(())
}
