#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

use std::env;

use anyhow::{Context, Result};
use semver::Version;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let input = get_input()?;

    add_mask(&input.actor);

    println!(
        "INPUTS: actor = {}, repo = {}, ref = {}",
        input.actor, input.repository, input.git_ref
    );

    let versions = create_versions(&input.git_ref).context("failed to create version list")?;
    let tags = generate_tags(&input.repository, &versions, &input.registries);

    set_output("tags", &tags);

    Ok(())
}

fn add_mask(value: &str) {
    println!("::add-mask::{}", value);
}

fn set_output(key: &str, value: &str) {
    println!("::set-output name={}::{}", key, value);
}

struct Input {
    actor: String,
    repository: String,
    git_ref: String,
    registries: Vec<String>,
}

fn get_input() -> Result<Input> {
    Ok(Input {
        actor: env::var("GITHUB_ACTOR")
            .context("failed loading GITHUB_ACTOR environment variable")?,
        repository: env::var("GITHUB_REPOSITORY")
            .context("failed loading GITHUB_REPOSITORY environment variable")?,
        git_ref: env::var("GITHUB_REF")
            .context("failed loading GITHUB_REF environment variable")?,
        registries: env::args()
            .nth(1)
            .context("failed loading registries from program arguments")?
            .split_whitespace()
            .map(str::to_owned)
            .collect(),
    })
}

fn create_versions(git_ref: &str) -> Result<Vec<String>> {
    const MAIN_BRANCH_REFS: &[&str] = &["refs/heads/main", "refs/heads/master"];

    Ok(if let Some(version) = extract_version(git_ref) {
        let semver = Version::parse(version).context("failed parsing semantic version")?;

        if semver.pre.is_empty() {
            vec![
                format!("{}", semver.major),
                format!("{}.{}", semver.major, semver.minor),
                format!("{}.{}.{}", semver.major, semver.minor, semver.patch),
            ]
        } else {
            vec![semver.to_string()]
        }
    } else if MAIN_BRANCH_REFS.contains(&git_ref) {
        vec!["latest".to_owned()]
    } else {
        vec![]
    })
}

fn extract_version(git_ref: &str) -> Option<&str> {
    const VERSION_REF: &str = "refs/tags/";

    git_ref.strip_prefix(VERSION_REF).and_then(|version| {
        version
            .find(|c: char| c.is_ascii_digit())
            .map(|idx| &version[idx..])
    })
}

fn generate_tags(
    repository: &str,
    versions: &[impl AsRef<str>],
    registries: &[impl AsRef<str>],
) -> String {
    registries
        .iter()
        .flat_map(|r| {
            versions
                .iter()
                .map(move |v| format!("{}/{}:{}", r.as_ref(), repository, v.as_ref()))
        })
        .fold(String::new(), |mut s, v| {
            if !s.is_empty() {
                s.push(',');
            }
            s.push_str(&v);
            s
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_versions() {
        assert_eq!(
            vec!["latest".to_owned()],
            super::create_versions("refs/heads/main").unwrap()
        );
        assert_eq!(
            vec!["latest".to_owned()],
            super::create_versions("refs/heads/master").unwrap()
        );
        assert_eq!(
            Vec::<String>::new(),
            super::create_versions("refs/heads/something").unwrap()
        );
        assert_eq!(
            vec!["1".to_owned(), "1.2".to_owned(), "1.2.3".to_owned()],
            super::create_versions("refs/tags/v1.2.3").unwrap()
        );
        assert_eq!(
            vec!["1.2.3-pre".to_owned()],
            super::create_versions("refs/tags/v1.2.3-pre").unwrap()
        );
        assert_eq!(
            vec!["1".to_owned(), "1.0".to_owned(), "1.0.0".to_owned()],
            super::create_versions("refs/tags/v1.0.0+abc").unwrap()
        );
        assert!(super::create_versions("refs/tags/v1a").is_err());
    }

    #[test]
    fn extract_version() {
        assert_eq!(Some("1.0.0"), super::extract_version("refs/tags/1.0.0"));
        assert_eq!(Some("1.0.0"), super::extract_version("refs/tags/v1.0.0"));
        assert_eq!(
            Some("1.0.0"),
            super::extract_version("refs/tags/abcde1.0.0")
        );
        assert_eq!(None, super::extract_version("1.0.0"));
        assert_eq!(None, super::extract_version("v1.0.0"));
        assert_eq!(None, super::extract_version("refs/heads/main"));
    }

    #[test]
    fn generate_tags() {
        assert_eq!(
            "docker.io/a:latest",
            super::generate_tags("a", &["latest"], &["docker.io"])
        );
        assert_eq!(
            "docker.io/a:1,docker.io/a:1.0,docker.io/a:1.0.0",
            super::generate_tags("a", &["1", "1.0", "1.0.0"], &["docker.io"])
        );
        assert_eq!(
            "docker.io/a/b:latest,ghcr.io/a/b:latest",
            super::generate_tags("a/b", &["latest"], &["docker.io", "ghcr.io"])
        );
    }
}
