use crate::cli::{OutputFormat, Process, ThawOptions};
use crate::lockfile::AutoDeserialize;
use crate::lockfile::v0::LockfileV0;
use crate::lockfile::v1::LockfileV1;
use anyhow::{Context, bail};
use core::fmt::Debug;
use owo_colors::OwoColorize;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct OnlyVersion {
    // only load version first. Other fields may change but this will remain the same.
    version: i8,
}

pub trait Thaw {
    async fn thaw(
        options: &ThawOptions,
        data: &[u8],
        format: OutputFormat,
    ) -> anyhow::Result<i32>
    where
        Self: Sized + Debug + DeserializeOwned;
}

impl Process for ThawOptions {
    async fn process(self) -> anyhow::Result<i32> {
        let contents = tokio::fs::read(&self.filename).await.with_context(|| {
            format!(
                "Failed to determine lockfile version in {}",
                self.filename.red()
            )
        })?;

        if let Some((version, format)) = OnlyVersion::auto(&contents) {
            match version {
                OnlyVersion { version: 0 } => LockfileV0::thaw(&self, &contents, format).await,
                OnlyVersion { version: 1 } => LockfileV1::thaw(&self, &contents, format).await,
                OnlyVersion { .. } => {
                    bail!("Unsupported version!")
                },
            }
        } else {
            bail!("Could not determine filetype of {}.", self.filename.red());
        }
    }
}
