pub mod alacritty;
pub mod hyprland;
pub mod mako;
pub mod swaylock;
pub mod waybar;
pub mod wofi;

#[macro_export]
macro_rules! create_config_dir_and_file {
    ($dir:expr, $conf:expr) => {
        return {
            let mut cfg = lib_hype::core::dirs::ConfigDirectory::get_create_config_subdir($dir).expect("No write access");
            cfg.push($conf);
            cfg
        }
    };
}