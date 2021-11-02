use std::path::PathBuf;

use reqwest::Url;

use crate::util::path_util;

#[tokio::main]
#[test]
async fn test_mvn_download() {
    let test_source = vec![
        "https://repo1.maven.org/maven2/".to_string(),
        "https://jitpack.io/".to_string(),
        "https://maven.icedlab.tech/proxy/".to_string(),
        "https://maven.aliyun.com/repository/gradle-plugin/".to_string(),
        "https://maven.fabricmc.net/".to_string(),
    ];
    let x = Box::new("fabric-loom/fabric-loom.gradle.plugin/0.9-SNAPSHOT/maven-metadata.xml");

    for s in test_source {
        let r = reqwest::get(Url::parse(s.as_str()).unwrap().join(x.as_ref()).unwrap()).await;
        println!(
            "request: {}",
            Url::parse(s.as_str()).unwrap().join(x.as_ref()).unwrap()
        );

        match r {
            Ok(x) => {
                println!("{}\n", x.status())
            }
            Err(x) => {
                println!("{}\n", x)
            }
        }
    }
}
