#![allow(unused)]

use std::sync::{atomic::{Ordering, AtomicBool}, LazyLock};

pub mod fs;
pub mod process;

static INSTANCE: LazyLock<FakeSystem> = LazyLock::new(FakeSystem::default);

#[derive(Debug)]
pub enum RealOrFake<R, F> {
    Real(R),
    Fake(F),
}


#[derive(Debug, Default)]
pub struct FakeSystem {
    enabled: AtomicBool,
    filesystem: fs::FakeFilesystem,
    processes: process::FakeProcesses,
}

impl FakeSystem {
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Release);
    }
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }
    pub fn fs(&self) -> &fs::FakeFilesystem {
        &self.filesystem
    }
    pub fn processes(&self) -> &process::FakeProcesses {
        &self.processes
    }
}

pub fn get() -> Option<&'static FakeSystem> {
    if INSTANCE.enabled() {
        Some(&INSTANCE)
    } else {
        None
    }
}