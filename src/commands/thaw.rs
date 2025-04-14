use crate::cli::{Process, ThawOptions};

impl Process for ThawOptions {
    async fn process(self) -> anyhow::Result<i32> {
        dbg!(self);

        Ok(0)
    }
}
