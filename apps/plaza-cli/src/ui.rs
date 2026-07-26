use indicatif::ProgressBar;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

pub struct TerminalUi;

impl TerminalUi {
    pub fn new() -> Self {
        Self
    }
    
    pub fn create_progress_bar(&self, _len: u64) -> ProgressBar {
        ProgressBar::hidden() // DP1 Stub
    }
    
    pub fn confirm(&self, prompt: &str) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(true)
            .interact()
            .unwrap_or(false)
    }
    
    pub fn select(&self, prompt: &str, items: &[&str]) -> Option<usize> {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(items)
            .default(0)
            .interact_opt()
            .unwrap_or(None)
    }
}
