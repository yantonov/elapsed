pub const NAME: &str = env!("CARGO_PKG_NAME");

/// The package version together with the full hash of the commit the binary
/// was built from ("unknown" when it was not available at build time).
/// Used both by the 'version' command and by the --version flag,
/// so the two never disagree.
pub const VERSION_WITH_COMMIT_HASH: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_COMMIT_HASH"),
    ")"
);

pub fn version_info() -> String {
    format!("{} {}", NAME, VERSION_WITH_COMMIT_HASH)
}
