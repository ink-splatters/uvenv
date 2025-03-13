use owo_colors::OwoColorize;
use crate::cli::{Process, SelfVersionOptions};
use crate::commands::self_info::self_info;

impl Process for SelfVersionOptions {
    async fn process(self) -> anyhow::Result<i32> {
        eprintln!(
            "{}: {} is deprecated in favor of {}.",
            "Warning".yellow(),
            "`self version`".red(),
            "`self info`".green()
        );
        self_info().await
    }
}
