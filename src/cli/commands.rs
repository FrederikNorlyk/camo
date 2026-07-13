use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "camo", about = "Camo Theme Picker", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Theme {
        name: String,
    },
    Wallpaper {
        #[command(subcommand)]
        action: WallpaperAction,
    },
}

#[derive(Subcommand)]
pub enum WallpaperAction {
    Reload,
}
