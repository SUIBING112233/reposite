use std::path::{Path, PathBuf};

use reqwest::Url;

use crate::util::url_util;

#[test]
fn test_1_pathbuf_url_helper() {
    let test_source = vec![
        "https://repo1.maven.org/maven2/".to_string(),
        "https://jitpack.io/".to_string(),
        "https://maven.icedlab.tech/proxy/".to_string(),
        "https://maven.aliyun.com/repository/gradle-plugin/".to_string(),
        "https://maven.fabricmc.net/".to_string(),
    ];

    let test_addition = "fabric-loom/fabric-loom.gradle.plugin/0.9-SNAPSHOT/maven-metadata.xml";
    let test_addition_pathbuf = PathBuf::new()
        .join("fabric-loom")
        .join("fabric-loom.gradle.plugin")
        .join("0.9-SNAPSHOT")
        .join("maven-metadata.xml");

    // ////////////////////////////////////////////
    // println!(
    //     "{:25}{}",
    //     "test_addition_pathbuf:",
    //     test_addition_pathbuf.to_str().unwrap()
    // );
    // println!("{:25}{}", "test_addition:", test_addition);
    // ////////////////////////////////////////////

    for index in test_source {
        let x1 = Url::parse(index.clone().as_str())
            .unwrap()
            .join(test_addition.clone())
            .unwrap();

        let x2 = Url::parse(index.clone().as_str())
            .unwrap()
            .join(
                url_util::pathbuf_url_helper(index.clone(), test_addition_pathbuf.clone()).as_str(),
            )
            .unwrap();

        assert_eq!(x1, x2);
    }
}

#[test]
fn test_2_pathbuf_url_helper() {
    let target = "https://repo1.maven.org/maven2/";
    let path = PathBuf::new()
        .join("fabric-loom")
        .join("fabric-loom.gradle.plugin")
        .join("0.9-SNAPSHOT")
        .join("maven-metadata.xml");

    let test="https://repo1.maven.org/maven2/fabric-loom/fabric-loom.gradle.plugin/0.9-SNAPSHOT/maven-metadata.xml";
    assert_eq!(
        url_util::pathbuf_url_helper(target.to_string(), path).as_str(),
        test
    );
}
