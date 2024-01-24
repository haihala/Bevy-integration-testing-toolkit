use bevy::prelude::*;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Resource)]
pub(crate) struct ArtefactPaths {
    pub(crate) base: PathBuf,
    pub(crate) running_headless: bool,
}

impl ArtefactPaths {
    pub fn pre_assert_screenshot(&self) -> PathBuf {
        self.base.join("pre-assert.png")
    }

    pub fn post_assert_screenshot(&self) -> PathBuf {
        self.base.join("post-assert.png")
    }

    pub fn frame_metrics(&self) -> PathBuf {
        self.base.join("frame_metrics.json")
    }

    pub fn saved(&self) -> bool {
        (self.running_headless || Self::file_saved(self.pre_assert_screenshot()))
            && (self.running_headless || Self::file_saved(self.post_assert_screenshot()))
            && Self::file_saved(self.frame_metrics())
    }

    fn file_saved(path: PathBuf) -> bool {
        path.exists() && File::open(path.clone()).unwrap().metadata().unwrap().len() > 0
    }
}
