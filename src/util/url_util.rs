use std::path::PathBuf;

use reqwest::Url;

/// Convert request `PathBuf` and base url into a new url.
/// # Example
/// ```edition2021
/// let target = "https://repo1.maven.org/maven2/";
/// let path = PathBuf::new()
///   .join("fabric-loom")
///   .join("fabric-loom.gradle.plugin")
///   .join("0.9-SNAPSHOT")
///   .join("maven-metadata.xml");
///
/// let test="https://repo1.maven.org/maven2/fabric-loom/fabric-loom.gradle.plugin/0.9-SNAPSHOT/maven-metadata.xml";
/// assert_eq!(
///    url_util::pathbuf_url_helper(target.to_string(), path).as_str(),
///    test
/// );
/// ```
pub fn pathbuf_url_helper(target: String, path: PathBuf) -> Url {
    Url::parse(&target)
        .unwrap()
        .join(path.into_os_string().to_str().unwrap())
        .unwrap()
}
