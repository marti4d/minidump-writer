use tempfile as real;
use fake_system::RealOrFake;

#[derive(Debug)]
pub struct NamedTempFile(RealOrFake<real::NamedTempFile, (std::path::PathBuf, std::fs::File)>);

impl NamedTempFile {
    pub fn path(&self) -> &std::path::Path {
        match &self.0 {
            RealOrFake::Real(r) => std::path::Path::from_std_path(r.path()),
            RealOrFake::Fake(f) => &f.0,
        }
    }
}

impl std::io::Write for NamedTempFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            RealOrFake::Real(r) => r.write(buf),
            RealOrFake::Fake(f) => f.1.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.0 {
            RealOrFake::Real(r) => r.flush(),
            RealOrFake::Fake(f) => f.1.flush(),
        }
    }
}

impl std::io::Seek for NamedTempFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match &mut self.0 {
            RealOrFake::Real(r) => r.seek(pos),
            RealOrFake::Fake(f) => f.1.seek(pos),
        }
    }
}

#[derive(Debug)]
pub struct TempDir(RealOrFake<real::TempDir, std::path::PathBuf>);

impl AsRef<std::path::Path> for TempDir {
    fn as_ref(&self) -> &std::path::Path {
        match &self.0 {
            RealOrFake::Real(r) => std::path::Path::from_std_path(r.as_ref()),
            RealOrFake::Fake(f) => f.as_ref(),
        }
    }
}

#[derive(Debug)]
pub struct Builder<'a, 'b> {
    prefix: Option<&'a std::ffi::OsStr>,
    suffix: Option<&'b std::ffi::OsStr>,
}

impl<'a, 'b> Builder<'a, 'b> {
    pub fn new() -> Self {
        Builder {
            prefix: None,
            suffix: None,
        }
    }
    pub fn prefix<S: AsRef<std::ffi::OsStr> + ?Sized>(&mut self, prefix: &'a S) -> &mut Self {
        self.prefix = Some(prefix.as_ref());
        self
    }
    pub fn suffix<S: AsRef<std::ffi::OsStr> + ?Sized>(&mut self, suffix: &'b S) -> &mut Self {
        self.suffix = Some(suffix.as_ref());
        self
    }
    pub fn tempfile(&self) -> std::io::Result<NamedTempFile> {
        if let Some(fake) = fake_system::get() {
            use rand::Rng;

            let mut name_ostr = std::ffi::OsString::new();
            if let Some(prefix) = self.prefix {
                name_ostr.push(prefix);
            }

            let mut stem_bytes = [0u8; 10];
            rand::thread_rng().fill(&mut stem_bytes);
            for b in stem_bytes.iter_mut() {
                let adjusted = b'a' + *b % 26;
                *b = adjusted;
            }
            let stem = std::str::from_utf8(&stem_bytes).unwrap();
            name_ostr.push(stem);

            if let Some(suffix) = self.suffix {
                name_ostr.push(suffix);
            }
            
            let path = std::path::PathBuf::from_real(std::env::temp_dir().join(name_ostr));
            std::fs::File::create(&path).map(|file| RealOrFake::Fake((path, file))).map(NamedTempFile)
        } else {
            let mut builder = real::Builder::new();
            if let Some(prefix) = self.prefix {
                builder.prefix(prefix);
            }
            if let Some(suffix) = self.suffix {
                builder.suffix(suffix);
            }
            builder.tempfile().map(RealOrFake::Real).map(NamedTempFile)
        }
    }
    pub fn tempdir(&self) -> std::io::Result<TempDir> {
        if let Some(fake) = fake_system::get() {
            use rand::Rng;

            let mut name_ostr = std::ffi::OsString::new();
            if let Some(prefix) = self.prefix {
                name_ostr.push(prefix);
            }

            let mut stem_bytes = [0u8; 10];
            rand::thread_rng().fill(&mut stem_bytes);
            for b in stem_bytes.iter_mut() {
                let adjusted = b'a' + *b % 26;
                *b = adjusted;
            }
            let stem = std::str::from_utf8(&stem_bytes).unwrap();
            name_ostr.push(stem);

            if let Some(suffix) = self.suffix {
                name_ostr.push(suffix);
            }
            
            let path = std::path::PathBuf::from_real(std::env::temp_dir().join(name_ostr));
            todo!();
        } else {
            let mut builder = real::Builder::new();
            if let Some(prefix) = self.prefix {
                builder.prefix(prefix);
            }
            if let Some(suffix) = self.suffix {
                builder.suffix(suffix);
            }
            builder.tempdir().map(RealOrFake::Real).map(TempDir)
        }
    }
}
