use std as real;

pub mod fs {
    use super::real::fs as real;
    use fake_system::RealOrFake;
    pub fn copy<P: AsRef<crate::path::Path>, Q: AsRef<crate::path::Path>>(
        from: P,
        to: Q,
    ) -> crate::io::Result<u64> {
        if let Some(fake) = fake_system::get() {
            fake.fs().copy(from.as_ref().as_std_path(), to.as_ref().as_std_path())
        } else {
            real::copy(from.as_ref().as_std_path(), to.as_ref().as_std_path())
        }
    }
    pub fn metadata<P: AsRef<crate::path::Path>>(
        path: P,
    ) -> crate::io::Result<crate::fs::Metadata> {
        if let Some(fake) = fake_system::get() {
            fake.fs().metadata(path.as_ref().as_std_path()).map(fake_system::RealOrFake::Fake)
        } else {
            real::metadata(path.as_ref().as_std_path()).map(fake_system::RealOrFake::Real)
        }
        .map(Metadata)
    }
    pub fn read<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<Vec<u8>> {
        if let Some(fake) = fake_system::get() {
            fake.fs().read(path.as_ref().as_std_path())
        } else {
            real::read(path.as_ref().as_std_path())
        }
    }
    pub fn read_dir<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<crate::fs::ReadDir> {
        if let Some(fake) = fake_system::get() {
            fake.fs().read_dir(path.as_ref().as_std_path()).map(fake_system::RealOrFake::Fake)
        } else {
            real::read_dir(path.as_ref().as_std_path()).map(fake_system::RealOrFake::Real)
        }
        .map(ReadDir)
    }
    pub fn read_link<P: AsRef<crate::path::Path>>(
        path: P,
    ) -> crate::io::Result<crate::path::PathBuf> {
        if let Some(fake) = fake_system::get() {
            fake.fs().read_link(path.as_ref().as_std_path()).map(crate::path::PathBuf)
        } else {
            real::read_link(path.as_ref().as_std_path()).map(crate::path::PathBuf)
        }
    }
    pub fn read_to_string<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<String> {
        if let Some(fake) = fake_system::get() {
            fake.fs().read_to_string(path.as_ref().as_std_path())
        } else {
            real::read_to_string(path.as_ref().as_std_path())
        }
    }
    pub fn remove_file<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<()> {
        if let Some(fake) = fake_system::get() {
            fake.fs().remove_file(path.as_ref().as_std_path())
        } else {
            real::remove_file(path.as_ref().as_std_path())
        }
    }

    #[derive(Debug)]
    pub struct Metadata(fake_system::RealOrFake<real::Metadata, fake_system::fs::Metadata>);

    impl Metadata {
        pub fn len(&self) -> u64 {
            use fake_system::RealOrFake;
            match &self.0 {
                RealOrFake::Real(r) => r.len(),
                RealOrFake::Fake(f) => f.len(),
            }
        }
    }

    #[derive(Debug)]
    pub struct ReadDir(fake_system::RealOrFake<real::ReadDir, fake_system::fs::ReadDir>);

    impl Iterator for ReadDir {
        type Item = crate::io::Result<DirEntry>;
        fn next(&mut self) -> Option<Self::Item> {
            use fake_system::RealOrFake;
            Some(match &mut self.0 {
                RealOrFake::Real(r) => r.next()?.map(fake_system::RealOrFake::Real),
                RealOrFake::Fake(f) => f.next()?.map(fake_system::RealOrFake::Fake),
            }
            .map(DirEntry))
        }
    }

    #[derive(Debug)]
    pub struct DirEntry(fake_system::RealOrFake<real::DirEntry, fake_system::fs::DirEntry>);

    impl DirEntry {
        pub fn file_name(&self) -> crate::ffi::OsString {
            use fake_system::RealOrFake;
            match &self.0 {
                RealOrFake::Real(r) => r.file_name(),
                RealOrFake::Fake(f) => f.file_name(),
            }
        }
        pub fn path(&self) -> crate::path::PathBuf {
            use fake_system::RealOrFake;
            match &self.0 {
                RealOrFake::Real(r) => crate::path::PathBuf(r.path()),
                RealOrFake::Fake(f) => crate::path::PathBuf(f.path()),
            }
        }
    }

    #[derive(Debug)]
    pub struct File(RealOrFake<real::File, fake_system::fs::File>);

    impl crate::os::unix::fs::FileExt for File {
        fn read_at(&self, buf: &mut [u8], offset: u64) -> crate::io::Result<usize> {
            match &self.0 {
                RealOrFake::Real(r) => <real::File as crate::os::unix::fs::FileExt>::read_at(r, buf, offset),
                RealOrFake::Fake(f) => f.read_at(buf, offset),
            }
            
        }
        fn write_at(&self, buf: &[u8], offset: u64) -> crate::io::Result<usize> {
            match &self.0 {
                RealOrFake::Real(r) => <real::File as crate::os::unix::fs::FileExt>::write_at(r, buf, offset),
                RealOrFake::Fake(f) => f.write_at(buf, offset),
            }
            
        }

        fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> crate::io::Result<()> {
            match &self.0 {
                RealOrFake::Real(r) => <real::File as crate::os::unix::fs::FileExt>::read_exact_at(r, buf, offset),
                RealOrFake::Fake(f) => f.read_exact_at(buf, offset),
            }
        }
        fn write_all_at(&self, buf: &[u8], offset: u64) -> crate::io::Result<()> {
            match &self.0 {
                RealOrFake::Real(r) => <real::File as crate::os::unix::fs::FileExt>::write_all_at(r, buf, offset),
                RealOrFake::Fake(f) => f.write_all_at(buf, offset),
            }
            
        }
    }

    impl memmap2::MmapAsRawDesc for &File {
        fn as_raw_desc(&self) -> memmap2::MmapRawDescriptor {
            match &self.0 {
                RealOrFake::Real(r) => <&real::File as memmap2::MmapAsRawDesc>::as_raw_desc(&r),
                RealOrFake::Fake(_f) => todo!(), // ¯\_(ツ)_/¯
            }
            
        }
    }

    impl File {
        pub fn create<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<File> {
            if let Some(fake) = fake_system::get() {
                fake.fs().create_file(path.as_ref().as_std_path()).map(RealOrFake::Fake)
            } else {
                real::File::create(path.as_ref().as_std_path()).map(RealOrFake::Real)
            }
            .map(File)
        }
        pub fn open<P: AsRef<crate::path::Path>>(path: P) -> crate::io::Result<File> {
            if let Some(fake) = fake_system::get() {
                fake.fs().open_file(path.as_ref().as_std_path()).map(RealOrFake::Fake)
            } else {
                real::File::open(path.as_ref().as_std_path()).map(RealOrFake::Real)
            }
            .map(File)
        }
    }

    impl crate::io::Read for File {
        fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.read(buf),
                RealOrFake::Fake(f) => f.read(buf),
            }
        }
    }

    impl crate::io::Write for File {
        fn write(&mut self, buf: &[u8]) -> crate::io::Result<usize> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.write(buf),
                RealOrFake::Fake(f) => f.write(buf),
            }
        }
        fn flush(&mut self) -> crate::io::Result<()> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.flush(),
                RealOrFake::Fake(f) => f.flush(),
            }
        }
    }

    impl crate::io::Seek for File {
        fn seek(&mut self, pos: crate::io::SeekFrom) -> crate::io::Result<u64> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.seek(pos),
                RealOrFake::Fake(f) => f.seek(pos),
            }
        }
    }

    impl crate::os::fd::AsFd for File {
        fn as_fd(&self) -> crate::os::fd::BorrowedFd<'_> {
            match &self.0 {
                RealOrFake::Real(r) => r.as_fd(),
                RealOrFake::Fake(f) => f.as_fd(),
            }
        }
    }
}

pub mod path {
    use super::real::path as real;
    pub use real::Display;

    #[derive(Debug, Eq, PartialEq)]
    pub struct PathBuf(pub(crate) real::PathBuf);

    impl PathBuf {
        pub fn push<P: AsRef<Path>>(&mut self, path: P) {
            self.0.push(&path.as_ref().0)
        }
        pub fn set_file_name<S: AsRef<crate::ffi::OsStr>>(&mut self, file_name: S) {
            self.0.set_file_name(file_name);
        }
        pub fn into_os_string(self) -> crate::ffi::OsString {
            self.0.into_os_string()
        }
        pub fn from_real(f: real::PathBuf) -> PathBuf {
            PathBuf(f)
        }
    }

    impl crate::ops::Deref for PathBuf {
        type Target = Path;
        fn deref(&self) -> &Self::Target {
            Path::from_std_path(self.0.deref())
        }
    }

    impl AsRef<crate::ffi::OsStr> for PathBuf {
        fn as_ref(&self) -> &crate::ffi::OsStr {
            self.0.as_ref()
        }
    }

    impl AsRef<Path> for PathBuf {
        fn as_ref(&self) -> &Path {
            <Self as crate::ops::Deref>::deref(self)
        }
    }
    impl<T: ?Sized + AsRef<crate::ffi::OsStr>> From<&T> for PathBuf {
        fn from(s: &T) -> PathBuf {
            PathBuf(real::PathBuf::from(s))
        }
    }
    impl From<String> for PathBuf {
        fn from(f: String) -> PathBuf {
            PathBuf(real::PathBuf::from(f))
        }
    }
    impl From<crate::ffi::OsString> for PathBuf {
        fn from(f: crate::ffi::OsString) -> PathBuf {
            PathBuf(real::PathBuf::from(f))
        }
    }
    impl crate::fmt::Display for PathBuf {
        fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
            write!(f, "{}", self.0.display())
        }
    }
    impl crate::borrow::Borrow<Path> for PathBuf {
        fn borrow(&self) -> &Path {
            Path::from_std_path(self.0.borrow())
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Path(real::Path);

    impl Path {
        pub fn new<S: AsRef<crate::ffi::OsStr> + ?Sized>(s: &S) -> &Path {
            Path::from_std_path(real::Path::new(s))
        }
        pub fn file_name(&self) -> Option<&crate::ffi::OsStr> {
            self.0.file_name()
        }
        pub fn is_dir(&self) -> bool {
            if let Some(fake) = fake_system::get() {
                fake.fs().is_dir(&self.0)
            } else {
                self.0.is_dir()
            }
        }
        pub fn as_os_str(&self) -> &crate::ffi::OsStr {
            self.0.as_os_str()
        }
        pub fn to_string_lossy(&self) -> crate::borrow::Cow<'_, str> {
            self.0.to_string_lossy()
        }
        pub fn exists(&self) -> bool {
            if let Some(fake) = fake_system::get() {
                fake.fs().exists(&self.0)
            } else {
                self.0.exists()
            }
        }
        pub fn display(&self) -> crate::path::Display<'_> {
            self.0.display()
        }
        pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
            PathBuf(self.0.join(path.as_ref().as_std_path()))
        }
        pub fn from_std_path(f: &real::Path) -> &Path {
            unsafe { crate::mem::transmute(f) }
        }
        pub fn as_std_path(&self) -> &real::Path {
            &self.0
        }
    }

    impl AsRef<Path> for Path {
        fn as_ref(&self) -> &Path {
            self
        }
    }

    impl AsRef<Path> for str {
        fn as_ref(&self) -> &Path {
            Path::from_std_path(self.as_ref())
        }
    }

    impl AsRef<Path> for String {
        fn as_ref(&self) -> &Path {
            Path::from_std_path(self.as_ref())
        }
    }

    impl AsRef<Path> for crate::ffi::OsString {
        fn as_ref(&self) -> &Path {
            Path::from_std_path(self.as_ref())
        }
    }

    impl ToOwned for Path {
        type Owned = PathBuf;
        fn to_owned(&self) -> PathBuf {
            PathBuf(self.0.to_owned())
        }
    }
}

pub mod os {
    use super::real::os as real;
    pub use real::fd;
    pub mod unix {
        use super::real::unix as real;
        pub use real::ffi;

        pub mod fs {
            use super::real::fs as real;
            pub use real::FileExt;
        }

        pub mod prelude {
            use super::real::prelude as real;
            pub use real::OsStrExt;
        }

        pub mod process {
            use super::real::process as real;
            pub use real::ExitStatusExt;
        }
    }
}

pub mod process {
    use super::real::process as real;
    pub use real::{ExitStatus, Output, Stdio};
    use fake_system::RealOrFake;

    #[derive(Debug)]
    pub struct Command(RealOrFake<real::Command, fake_system::process::Command>);

    impl Command {
        pub fn new<S: AsRef<crate::ffi::OsStr>>(program: S) -> Command {
            if let Some(fake) = fake_system::get() {
                Command(RealOrFake::Fake(fake.processes().create(program.as_ref())))
            } else {
                Command(RealOrFake::Real(real::Command::new(program)))
            }
        }
        pub fn arg<S: AsRef<crate::ffi::OsStr>>(&mut self, arg: S) -> &mut Command {
            match &mut self.0 {
                RealOrFake::Real(r) => { r.arg(arg); }
                RealOrFake::Fake(f) => { f.arg(arg); }
            }
            self
        }
        pub fn args<I, S>(&mut self, args: I) -> &mut Command
        where
            I: IntoIterator<Item = S>,
            S: AsRef<crate::ffi::OsStr>,
        {
            match &mut self.0 {
                RealOrFake::Real(r) => { r.args(args); }
                RealOrFake::Fake(f) => { f.args(args); }
            }
            self
        }
        pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
        where
            K: AsRef<crate::ffi::OsStr>,
            V: AsRef<crate::ffi::OsStr>,
        {
            match &mut self.0 {
                RealOrFake::Real(r) => { r.env(key, val); }
                RealOrFake::Fake(f) => { f.env(key, val); }
            }
            self
        }
        pub fn output(&mut self) -> crate::io::Result<Output> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.output(),
                RealOrFake::Fake(f) => f.output(),
            }
        }
        pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
            match &mut self.0 {
                RealOrFake::Real(r) => {r.stdout(cfg);}
                RealOrFake::Fake(f) => {f.stdout(cfg);}
            }
            self
        }
        pub fn spawn(&mut self) -> crate::io::Result<Child> {
            match &mut self.0 {
                RealOrFake::Real(r) => r.spawn().map(|mut inner| {
                    let stdout = inner.stdout.take().map(RealOrFake::Real).map(ChildStdout);
                    Child {
                        inner: RealOrFake::Real(inner),
                        stdout,
                    }
                }),
                RealOrFake::Fake(f) => f.spawn().map(|mut inner| {
                    let stdout = inner.stdout.take().map(RealOrFake::Fake).map(ChildStdout);
                    
                    Child {
                        inner: RealOrFake::Fake(inner),
                        stdout,
                    }
                }),
            }
            
        }
    }

    #[derive(Debug)]
    pub struct ChildStdout(RealOrFake<real::ChildStdout, fake_system::process::ChildStdout>);

    impl crate::io::Read for ChildStdout {
        fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
            match &mut self.0 {
                RealOrFake::Real(r) => {r.read(buf)}
                RealOrFake::Fake(f) => {f.read(buf)}
            }
        }
    }

    #[derive(Debug)]
    pub struct Child {
        inner: RealOrFake<real::Child, fake_system::process::Child>,
        pub stdout: Option<ChildStdout>,
    }

    impl Child {
        pub fn id(&self) -> u32 {
            match &self.inner {
                RealOrFake::Real(r) => r.id(),
                RealOrFake::Fake(f) => f.id(),
            }
        }
        pub fn kill(&mut self) -> crate::io::Result<()> {
            match &mut self.inner {
                RealOrFake::Real(r) => r.kill(),
                RealOrFake::Fake(f) => f.kill(),
            }
        }
        pub fn wait(&mut self) -> crate::io::Result<ExitStatus> {
            match &mut self.inner {
                RealOrFake::Real(r) => r.wait(),
                RealOrFake::Fake(f) => f.wait(),
            }
        }
    }

    pub fn id() -> u32 {
        if let Some(fake) = fake_system::get() {
            fake.processes().current_process_id()
        } else {
            real::id()
        }
    }
}

pub use real::{
    alloc, array, assert_eq, borrow, cmp, collections, convert, env, eprintln, error, ffi, fmt,
    format, io, iter, marker, matches, mem, num, ops, panic, prelude, println, ptr, result, slice,
    string, sync, thread, time, todo, vec, write, writeln, str
};
