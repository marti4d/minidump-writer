use procfs_core as real;

pub use real::{FromRead as RealFromRead, ProcError};

pub mod process {
    use super::real::process as real;
    pub use real::{MMPermissions, MMapPath, MemoryMaps, ProcState, Stat};
}

pub trait FromRead: RealFromRead + Sized {
    fn from_read<R: std::io::Read>(r: R) -> real::ProcResult<Self> {
        <Self as RealFromRead>::from_read(r)
    }
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> real::ProcResult<Self> {
        let contents = std::fs::read(path).map_err(|e| 
            real::ProcError::Io(e, None)
        )?;
        <Self as FromRead>::from_read(contents.as_slice())
    }
}

impl FromRead for process::MemoryMaps {}
impl FromRead for process::Stat {}
