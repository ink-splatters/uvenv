use crate::cli::{FreezeOptions, OutputFormat};
use crate::metadata::Metadata;
use core::fmt::Debug;
use owo_colors::OwoColorize;
use regex::Regex;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub mod v0;
pub mod v1;

type PackageMap<P> = BTreeMap<String, P>;

trait PackageSpec: From<Metadata> {}

trait Lockfile<'de, P: PackageSpec + From<Metadata> + Debug + Serialize> {
    fn new(packages: PackageMap<P>) -> Self;

    async fn serialize_and_patch(
        &self,
        options: &FreezeOptions,
    ) -> anyhow::Result<Vec<u8>>
    where
        Self: Sized + Serialize;

    // predefined implementations:

    async fn dump_to_file(
        &self,
        options: &FreezeOptions,
    ) -> anyhow::Result<()>
    where
        Self: Sized + Serialize,
    {
        let format = &options.format;
        let filename = &options.filename;

        let serialized = self.serialize_and_patch(options).await?;

        let mut file = File::create(filename).await?;
        file.write_all(&serialized).await?;

        eprintln!(
            "Saved {} to {}.",
            format.to_string().blue(),
            filename.green()
        );

        Ok(())
    }

    async fn write(
        packages: PackageMap<P>,
        options: &FreezeOptions,
    ) -> anyhow::Result<bool>
    where
        Self: Sized + Debug + Serialize,
    {
        let instance = Self::new(packages);
        instance.dump_to_file(options).await?;
        Ok(true)
    }
}

pub trait AutoDeserialize: DeserializeOwned {
    fn from_json(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
    fn from_msgpack(data: &[u8]) -> Option<Self> {
        rmp_serde::decode::from_slice(data).ok()
    }
    fn from_toml(data: &[u8]) -> Option<Self> {
        let data_str = String::from_utf8(data.to_owned()).ok()?;
        toml::from_str(&data_str).ok()
    }

    fn from_format(
        data: &[u8],
        format: OutputFormat,
    ) -> Option<Self> {
        match format {
            OutputFormat::JSON => Self::from_json(data),
            OutputFormat::TOML => Self::from_toml(data),
            OutputFormat::Binary => Self::from_msgpack(data),
        }
    }

    fn auto(data: &[u8]) -> Option<(Self, OutputFormat)> {
        None /* Start with None so the rest or_else are all the same structure */
            .or_else(|| Self::from_json(data).map(|version| (version, OutputFormat::JSON)))
            .or_else(|| Self::from_msgpack(data).map(|version| (version, OutputFormat::Binary)))
            .or_else(|| Self::from_toml(data).map(|version| (version, OutputFormat::TOML)))
    }
}

fn extract_python_version(input: &str) -> Option<String> {
    let Ok(re) = Regex::new(r"(\d+)\.(\d+)") else {
        return None;
    };

    re.captures(input).map(|caps| {
        let major = &caps[1];
        let minor = &caps[2];
        format!("{major}.{minor}")
    })
}

impl<L: DeserializeOwned> AutoDeserialize for L {}
