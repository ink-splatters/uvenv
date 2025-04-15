
use crate::cli::{FreezeOptions, Process};
use crate::commands::list::list_packages;
use crate::metadata::{LoadMetadataConfig, Metadata, serialize_msgpack};
use anyhow::bail;
use core::fmt::Debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use owo_colors::OwoColorize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

type PackageMap<P> = BTreeMap<String, P>;


trait Lockfile<'de, P: PackageSpec + From<Metadata> + Debug + Serialize> {
    fn new(packages: PackageMap<P>) -> Self;

    async fn serialize_and_patch(
        &self,
        options: &FreezeOptions
    ) -> anyhow::Result<Vec<u8>>
    where
        Self: Sized + Serialize;

    // predefined implementations:

    async fn dump_to_file(
        &self,
        options: &FreezeOptions
    ) -> anyhow::Result<()>
    where
        Self: Sized + Serialize,
    {
        let format = &options.format;
        let filename = &options.filename;
        
        let serialized = self.serialize_and_patch(options).await?;

        let mut file = File::create(filename).await?;
        file.write_all(&serialized).await?;

        eprintln!("Saved {} to {}.", format.blue(), filename.green());
        
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
        instance
            .dump_to_file(options)
            .await?;
        Ok(true)
    }

    async fn process(options: &FreezeOptions) -> anyhow::Result<i32>
    where
        Self: Sized + Debug + Serialize,
    {
        let pkg_metadata = list_packages(&LoadMetadataConfig::none(), None, None).await?;

        let packages: PackageMap<P> = if !options.include.is_empty() {
            // --include passed
            pkg_metadata
                .into_iter()
                .filter_map(|meta| {
                    options
                        .include
                        .contains(&meta.name)
                        .then(|| (meta.name.clone(), meta.into()))
                })
                .collect()
        } else if !options.exclude.is_empty() {
            // --exclude passed
            pkg_metadata
                .into_iter()
                .filter_map(|meta| {
                    if options.exclude.contains(&meta.name) {
                        None
                    } else {
                        Some((meta.name.clone(), meta.into()))
                    }
                })
                .collect()
        } else {
            // just do all
            pkg_metadata
                .into_iter()
                .map(|meta| (meta.name.clone(), meta.into()))
                .collect()
        };

        Ok(Self::write(packages, options).await?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[cfg(debug_assertions)]
struct PackageSpecV0;

#[cfg(debug_assertions)]
impl From<Metadata> for PackageSpecV0 {
    fn from(_: Metadata) -> Self {
        Self { }
    }
}

#[cfg(debug_assertions)]
impl PackageSpec for PackageSpecV0 {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[cfg(debug_assertions)]
struct LockfileV0 {
    version: i8
}

#[cfg(debug_assertions)]
impl Lockfile<'_, PackageSpecV0> for LockfileV0 {
    fn new(_: PackageMap<PackageSpecV0>) -> Self {
        Self {
            version: 0
        }
    }

    async fn serialize_and_patch(
        &self,
        options: &FreezeOptions
    ) -> anyhow::Result<Vec<u8>> {
        Ok(
            match options.format.as_ref() {
                "toml" => {
                    toml::to_string(self)?.into_bytes()
                },
                "json" => serde_json::to_string_pretty(self)?.into_bytes(),
                "binary" => serialize_msgpack(self).await?,
                other => {
                    bail!("Unsupported format {}", other);
                },
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct LockfileV1 {
    version: i8,
    packages: PackageMap<PackageSpecV1>,
}

impl Lockfile<'_, PackageSpecV1> for LockfileV1 {
    fn new(packages: PackageMap<PackageSpecV1>) -> Self {
        Self {
            version: 1,
            packages,
        }
    }

    async fn serialize_and_patch(
        &self,
        options: &FreezeOptions
    ) -> anyhow::Result<Vec<u8>> {
        let serialized = match options.format.as_ref() {
            "toml" => {
                // this `to_document` converts everything to inline tables:
                let mut doc = toml_edit::ser::to_document(self)?;

                // now convert all top-level tables from inline to regular:
                for (_, item) in doc.iter_mut() {
                    // Attempt to convert the inline table into a normal table.
                    // Here we use as_inline_table_mut; if the packages field is indeed an inline table,
                    // we can take it out and call .into_table() to convert it.
                    if let Some(inline_table) = item.as_inline_table_mut() {
                        // Replace the inline table with a block table.
                        // Note: std::mem::take clears the inline table, leaving an empty one behind.
                        let table = core::mem::take(inline_table).into_table();
                        *item = toml_edit::Item::Table(table);
                    }
                }

                doc.to_string().into_bytes()
            },
            "json" => serde_json::to_string_pretty(self)?.into_bytes(),
            "binary" => serialize_msgpack(self).await?,
            other => {
                bail!("Unsupported format {}", other);
            },
        };

        Ok(serialized)
    }
}

trait PackageSpec: From<Metadata> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
struct PackageSpecV1 {
    spec: String,
    version: String,
    python: Option<String>,
    injected: Vec<String>,
    editable: bool,
}

impl PackageSpec for PackageSpecV1 {}

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

impl From<Metadata> for PackageSpecV1 {
    fn from(value: Metadata) -> Self {
        let version = if value.requested_version.is_empty() {
            format!("~{}", value.installed_version)
        } else {
            value.requested_version
        };

        let python = extract_python_version(&value.python);

        let injected = value.injected.into_iter().collect();

        Self {
            spec: value.install_spec,
            editable: value.editable,
            version,
            python,
            injected,
        }
    }
}

static LATEST_VERSION: &str = "1";

impl Process for FreezeOptions {
    async fn process(self) -> anyhow::Result<i32> {
        let version = self.version.as_ref().map_or(LATEST_VERSION, |ver| ver);

        match version {
            #[cfg(debug_assertions)]
            "0" => LockfileV0::process(&self).await,
            "1" => LockfileV1::process(&self).await,
            _ => {
                bail!("Unsupported version!")
            },
        }
    }
}
