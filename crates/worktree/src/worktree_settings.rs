use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorktreeSettings {
    /// Completely ignore files matching globs from `file_scan_exclusions`
    ///
    /// Default: [
    ///   "**/.git",
    ///   "**/.svn",
    ///   "**/.hg",
    ///   "**/CVS",
    ///   "**/.DS_Store",
    ///   "**/Thumbs.db",
    ///   "**/.classpath",
    ///   "**/.settings"
    /// ]
    #[serde(default)]
    pub file_scan_exclusions: Option<Vec<String>>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: [ "**/.env*" ]
    pub private_files: Option<Vec<String>>,
}

impl Settings for WorktreeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
