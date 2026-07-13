use crate::services::themers::{ThemeContext, Themer};
use std::process::Command;

pub struct NoctaliaThemer;

impl Themer for NoctaliaThemer {
    fn apply(&self, context: &ThemeContext<'_>) -> Result<(), String> {
        let noctalia_theme = &context.theme.noctalia_theme;

        Command::new("noctalia")
            .arg("msg")
            .arg("color-scheme-set")
            .arg("builtin")
            .arg(noctalia_theme)
            .output()
            .map_err(|e| format!("Failed to set Noctalia theme: {e}"))?;

        Ok(())
    }
}
