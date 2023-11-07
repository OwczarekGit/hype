mod programs;
use lib_theme::core::themeable::Themeable;

fn main() {
    let theme = lib_theme::themes::solarized_dark::theme();
    programs::alacritty::Alacritty.apply(theme.clone());
    programs::wofi::Wofi.apply(theme.clone());
}
