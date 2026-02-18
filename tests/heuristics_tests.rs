use diskdoc::domain::entities::FileType;
use diskdoc::infrastructure::scanner::heuristics::HeuristicsEngine;
use std::path::PathBuf;

#[test]
fn test_log_detection() {
    let engine = HeuristicsEngine::new();

    assert_eq!(
        engine.analyze(&PathBuf::from("/var/log/syslog"), false),
        FileType::Log
    );
    assert_eq!(
        engine.analyze(&PathBuf::from("app.log"), false),
        FileType::Log
    );
    assert_eq!(
        engine.analyze(&PathBuf::from("image.png"), false),
        FileType::Normal
    );
}

#[test]
fn test_cache_detection() {
    let engine = HeuristicsEngine::new();

    assert_eq!(
        engine.analyze(
            &PathBuf::from("/Users/me/Library/Caches/com.apple.dt.Xcode"),
            true
        ),
        FileType::Cache
    );
    assert_eq!(
        engine.analyze(&PathBuf::from("/home/user/.cache/mozilla"), true),
        FileType::Cache
    );
    assert_eq!(
        engine.analyze(&PathBuf::from("/project/target/debug/build"), true),
        FileType::Cache
    );
}

#[test]
fn test_npm_detection() {
    let engine = HeuristicsEngine::new();
    assert_eq!(
        engine.analyze(&PathBuf::from("/home/user/.npm/_cacache/content-v2"), true),
        FileType::NpmCache
    );
    assert_eq!(
        engine.analyze(&PathBuf::from("/project/node_modules/.cache/babel"), true),
        FileType::NpmCache
    );
}

#[test]
fn test_composer_detection() {
    let engine = HeuristicsEngine::new();
    assert_eq!(
        engine.analyze(&PathBuf::from("/home/user/.composer/cache/repo"), true),
        FileType::ComposerCache
    );
}

#[test]
fn test_apt_detection() {
    let engine = HeuristicsEngine::new();
    assert_eq!(
        engine.analyze(&PathBuf::from("/var/cache/apt/archives/partial"), true),
        FileType::AptCache
    );
}

#[test]
fn test_docker_detection() {
    let engine = HeuristicsEngine::new();
    // This is hard to test without actual paths existing if we used fs::exists,
    // but our current heuristic just checks path string.
    assert_eq!(
        engine.analyze(&PathBuf::from("/var/lib/docker/overlay2"), true),
        FileType::Docker
    );
}
