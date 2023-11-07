mod programs;
mod themeable;
use themeable::Themeable;

fn main() {
    let theme = lib_theme::themes::solarized_dark::theme();
    programs::alacritty::Alacritty.apply(theme);
}