use crossterm::style::Color::{AnsiValue, Green, Rgb, Yellow};
use termimad::{rgb, MadSkin,StyledChar};

pub fn default() -> MadSkin {
    let mut skin= MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r:28,
        g:28,
        b:28,
    });

    skin.bullet = StyledChar::from_fg_char(Yellow, '')


}