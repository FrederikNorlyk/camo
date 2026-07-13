use clap::Parser;
use std::io;
use theme_picker::cli::commands::{Cli, Command, WallpaperAction};
use theme_picker::services::theme_service::ThemeService;
use theme_picker::tui::terminal_user_interface::TerminalUserInterface;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        ratatui::run(|terminal| TerminalUserInterface::default().run(terminal))?;
        return Ok(());
    };

    match command {
        Command::Theme { name } => {
            let themes = ThemeService::get_available_themes().unwrap_or_else(|e| {
                eprintln!("Could not get themes: {e}");
                Vec::new()
            });

            let Some(theme) = themes.iter().find(|theme| theme.name.eq(&name)) else {
                eprintln!("Could not get theme: {name}");
                eprintln!("Available themes:");
                for theme in themes {
                    eprintln!(" - {}", theme.name);
                }
                return Ok(());
            };

            match ThemeService::set_current_theme(theme) {
                Ok(()) => println!("The theme was set successfully"),
                Err(e) => eprintln!("Error setting theme: {e}"),
            }
        }
        Command::Wallpaper { action } => match action {
            WallpaperAction::Reload => match ThemeService::change_wallpaper() {
                Ok(()) => println!("The wallpaper was reloaded"),
                Err(e) => eprintln!("Error reloading wallpaper: {e}"),
            },
        },
    }

    Ok(())
}
