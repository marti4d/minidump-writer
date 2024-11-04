#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FakeProcesses {
    table: HashMap<libc::pid_t, Process>,
}

impl FakeProcesses {
    pub fn waitpid<P: Into<Option<nix::unistd::Pid>>>(
        &self,
        pid: P,
        options: Option<nix::sys::wait::WaitPidFlag>,
    ) -> nix::Result<nix::sys::wait::WaitStatus> {
        todo!();
    }
    pub fn kill<T: Into<Option<nix::sys::signal::Signal>>>(
        &self,
        pid: nix::unistd::Pid,
        signal: T,
    ) -> nix::Result<()> {
        todo!();
    }
    pub fn ptrace<A, B, C>(&self, request: libc::c_uint, a: A, b: B, c: C) -> libc::c_long {
        todo!();
    }
    pub fn ptrace_attach(&self, pid: nix::unistd::Pid) -> nix::Result<()> {
        todo!();
    }
    pub fn ptrace_cont<T: Into<Option<nix::sys::signal::Signal>>>(
        &self,
        pid: nix::unistd::Pid,
        sig: T,
    ) -> nix::Result<()> {
        todo!();
    }
    pub fn ptrace_detach<T: Into<Option<nix::sys::signal::Signal>>>(
        &self,
        pid: nix::unistd::Pid,
        sig: T,
    ) -> nix::Result<()> {
        todo!();
    }
    pub fn ptrace_read(&self, pid: nix::unistd::Pid, addr: nix::sys::ptrace::AddressType) -> nix::Result<libc::c_long> {
        todo!();
    }
    pub fn process_vm_readv(
        &self,
        pid: nix::unistd::Pid,
        local_iov: &mut [std::io::IoSliceMut<'_>],
        remote_iov: &[nix::sys::uio::RemoteIoVec],
    ) -> nix::Result<usize> {
        todo!();
    }
    pub fn mmap<F: std::os::fd::AsFd>(
        &self,
        addr: Option<std::num::NonZeroUsize>,
        length: std::num::NonZeroUsize,
        prot: nix::sys::mman::ProtFlags,
        flags: nix::sys::mman::MapFlags,
        f: F,
        offset: libc::off_t,
    ) -> nix::Result<std::ptr::NonNull<std::ffi::c_void>> {
        todo!();
    }
    pub fn mmap_anonymous(
        &self,
        addr: Option<std::num::NonZeroUsize>,
        length: std::num::NonZeroUsize,
        prot: nix::sys::mman::ProtFlags,
        flags: nix::sys::mman::MapFlags,
    ) -> nix::Result<std::ptr::NonNull<std::ffi::c_void>> {
        todo!();
    }
    pub fn getppid(&self) -> nix::unistd::Pid {
        todo!();
    }
    pub fn create(&self, program: &std::ffi::OsStr) -> Command {
        todo!();
    }
    pub fn current_process_id(&self) -> u32 {
        todo!();
    }
}

#[derive(Debug)]
pub struct Process {

}

#[derive(Debug)]
pub struct Command {
    program: std::ffi::OsString,
    args: Vec<std::ffi::OsString>,
    env: HashMap<std::ffi::OsString, std::ffi::OsString>,
    stdout: Option<std::process::Stdio>,
}

impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            program: program.as_ref().to_os_string(),
            args: Vec::new(),
            env: HashMap::new(),
            stdout: None,
        }
    }
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) -> &mut Command {
        self.args.push(arg.as_ref().to_os_string());
        self
    }
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.args.extend(args.into_iter().map(|s| s.as_ref().to_os_string()));
        self
    }
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        self.env.insert(key.as_ref().to_os_string(), val.as_ref().to_os_string());
        self
    }
    pub fn output(&mut self) -> std::io::Result<std::process::Output> {
        todo!();
    }
    pub fn stdout<T: Into<std::process::Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdout = Some(cfg.into());
        self
    }
    pub fn spawn(&mut self) -> std::io::Result<Child> {
        todo!();
    }
}

#[derive(Debug)]
pub struct Child {
    pub stdout: Option<ChildStdout>,
}

impl Child {
    pub fn id(&self) -> u32 {
        todo!();
    }
    pub fn kill(&mut self) -> std::io::Result<()> {
        todo!();
    }
    pub fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        todo!();
    }
}

#[derive(Debug)]
pub struct ChildStdout {}

impl ChildStdout {
    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!();
    }
}