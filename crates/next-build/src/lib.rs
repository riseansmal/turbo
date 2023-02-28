#![feature(min_specialization)]

pub mod build_options;

use anyhow::Result;
use turbo_tasks::{run_once, TurboTasks};
use turbo_tasks_memory::MemoryBackend;

use crate::build_options::BuildOptions;

pub async fn build(options: BuildOptions) -> Result<()> {
    #[cfg(feature = "tokio_console")]
    console_subscriber::init();
    register();

    let tt = TurboTasks::new(MemoryBackend::new(
        options.memory_limit.map_or(usize::MAX, |l| l * 1024 * 1024),
    ));

    run_once(tt, async move {
        eprintln!("running next-build");

        Ok(())
    })
    .await?;

    Ok(())
}

pub fn register() {
    turbo_tasks_fs::register();
    turbopack_core::register();
    next_core::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
