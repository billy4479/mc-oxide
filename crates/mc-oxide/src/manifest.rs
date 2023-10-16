use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::prelude::*;
use futures::future::join_all;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VersionManifest {
    pub id: String,
    pub jar_url: String,
    pub java_version: u64,
    pub sha1: String,
}

lazy_static! {
    pub static ref MANIFEST_PATH: PathBuf = CACHE_DIR.join("manifest.yml");
}

pub async fn get_version_infos() -> Result<Vec<VersionManifest>> {
    match fs::metadata(MANIFEST_PATH.as_path()) {
        Ok(_meta) => {
            // TODO: check expired manifest
            info!("Version manifests file was found, loading it");
            Ok(serde_yaml::from_reader(File::open(
                MANIFEST_PATH.as_path(),
            )?)?)
        }
        Err(e) => {
            warn!(
                "An error occurred while gathering metadata about the version manifest file: {}",
                e
            );

            update_manifest().await
        }
    }
}

pub async fn update_manifest() -> Result<Vec<VersionManifest>> {
    fs::create_dir_all(CACHE_DIR.as_path())?;
    let manifest = download_manifest().await?;

    serde_yaml::to_writer(File::create(MANIFEST_PATH.as_path())?, &manifest)?;
    Ok(manifest)
}

async fn download_manifest() -> Result<Vec<VersionManifest>> {
    const MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

    info!("downloading version manifest");

    let value: serde_json::Value =
        serde_json::from_str(reqwest::get(MANIFEST_URL).await?.text().await?.as_str())?;

    trace!("downloaded version list");

    let version_manifests_urls_lambda = move || {
        let mut result = vec![];

        for version in value["versions"].as_array()? {
            result.push(version.as_object()?["url"].as_str()?.to_string())
        }

        Some(result)
    };

    let version_manifests_urls = match version_manifests_urls_lambda() {
        Some(result) => Ok::<Vec<String>, Error>(result),
        None => Err(anyhow!("error parsing json")),
    }?;

    trace!("parsed version list, dispatching tasks now");

    let mut tasks = vec![];
    for manifest_url in version_manifests_urls {
        tasks.push(tokio::spawn(async move {
            let value: serde_json::Value =
                serde_json::from_str(reqwest::get(&manifest_url).await?.text().await?.as_str())?;

            trace!("request to {} was successful", manifest_url.as_str());

            let java_version = (|| {
                value
                    .get("javaVersion")?
                    .as_object()?
                    .get("majorVersion")?
                    .as_u64()
            })()
            .unwrap_or(8); // Minecraft versions before 1.7 don't have a Java version specified
                           // but it should be Java 8.

            let id = value["id"]
                .as_str()
                .ok_or_else(|| anyhow!("error parsing version id"))?
                .to_string();

            let (jar_url, sha1) = match (|| -> Option<(String, String)> {
                let server = value
                    .get("downloads")?
                    .as_object()?
                    .get("server")?
                    .as_object()?;
                let jar_url = server.get("url")?.as_str()?.to_string();
                let sha1 = server.get("sha1")?.as_str()?.to_string();

                Some((jar_url, sha1))
            })() {
                Some(tup) => tup,
                None => return Ok::<Option<VersionManifest>, Error>(None),
            };

            let manifest = VersionManifest {
                id,
                jar_url,
                java_version,
                sha1,
            };

            trace!("{} parsed", manifest.id);

            Ok(Some(manifest))
        }));
    }

    let mut result = vec![];

    for task in join_all(tasks).await {
        if let Some(v) = task?? {
            result.push(v)
        }
    }

    info!("download completed");
    Ok(result)
}
