use anyhow::Result;
use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() -> Result<()> {
    // Generate the default 'cargo:' instruction output
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    *config.git_mut().commit_timestamp_kind_mut() = TimestampKind::DateOnly;

    vergen(config)
}
