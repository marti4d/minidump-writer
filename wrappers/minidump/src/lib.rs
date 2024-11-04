use minidump as real;

pub use real::{
    MinidumpException, MinidumpHandleDataStream, MinidumpMemoryInfoList, MinidumpMemoryList,
    MinidumpModuleList, MinidumpSystemInfo, MinidumpThreadList, MinidumpThreadNames, Module,
};

#[derive(Debug)]
pub struct Minidump<'a, T>(real::Minidump<'a, T>)
where
    T: std::ops::Deref<Target = [u8]> + 'a;

pub type MmapMinidump = Minidump<'static, Vec<u8>>;

impl MmapMinidump {
    pub fn read_path<P>(path: P) -> Result<MmapMinidump, minidump::Error>
    where
        P: AsRef<std::path::Path>,
    {
        let contents = std::fs::read(path).or(Err(real::Error::FileNotFound))?;
        real::Minidump::read(contents).map(Minidump)
    }
}

impl<'a, T> Minidump<'a, T>
where
    T: std::ops::Deref<Target = [u8]> + 'a,
{
    pub fn get_raw_stream(&'a self, stream_type: u32) -> Result<&'a [u8], minidump::Error> {
        self.0.get_raw_stream(stream_type)
    }
    pub fn get_stream<S>(&'a self) -> Result<S, minidump::Error>
    where
        S: minidump::MinidumpStream<'a>,
    {
        self.0.get_stream()
    }
}
