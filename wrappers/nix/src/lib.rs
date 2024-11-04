use nix as real;

pub mod sys {
    use super::real::sys as real;
    pub use real::utsname;
    pub mod wait {
        use super::real::wait as real;
        pub use real::{WaitPidFlag, WaitStatus};
        pub fn waitpid<P: Into<Option<crate::unistd::Pid>>>(
            pid: P,
            options: Option<WaitPidFlag>,
        ) -> crate::Result<WaitStatus> {
            if let Some(fake) = fake_system::get() {
                fake.processes().waitpid(pid, options)
            } else {
                real::waitpid(pid, options)
            }
        }
    }
    pub mod signal {
        use super::real::signal as real;
        pub use real::{Signal, SIGCONT, SIGSTOP};
        pub fn kill<T: Into<Option<nix::sys::signal::Signal>>>(
            pid: crate::unistd::Pid,
            signal: T,
        ) -> crate::Result<()> {
            if let Some(fake) = fake_system::get() {
                fake.processes().kill(pid, signal)
            } else {
                real::kill(pid, signal)
            }
        }
    }
    pub mod ptrace {
        use super::real::ptrace as real;
        pub use real::{AddressType, Request, RequestType};
        pub fn attach(pid: crate::unistd::Pid) -> crate::Result<()> {
            if let Some(fake) = fake_system::get() {
                fake.processes().ptrace_attach(pid)
            } else {
                real::attach(pid)
            }
        }
        pub fn cont<T: Into<Option<nix::sys::signal::Signal>>>(
            pid: crate::unistd::Pid,
            sig: T,
        ) -> crate::Result<()> {
            if let Some(fake) = fake_system::get() {
                fake.processes().ptrace_cont(pid, sig)
            } else {
                real::cont(pid, sig)
            }
        }
        pub fn detach<T: Into<Option<nix::sys::signal::Signal>>>(
            pid: crate::unistd::Pid,
            sig: T,
        ) -> crate::Result<()> {
            if let Some(fake) = fake_system::get() {
                fake.processes().ptrace_detach(pid, sig)
            } else {
                real::detach(pid, sig)
            }
        }
        pub fn read(pid: crate::unistd::Pid, addr: AddressType) -> crate::Result<libc::c_long> {
            if let Some(fake) = fake_system::get() {
                fake.processes().ptrace_read(pid, addr)
            } else {
                real::read(pid, addr)
            }
        }
    }
    pub mod uio {
        use super::real::uio as real;
        pub use real::RemoteIoVec;
        pub fn process_vm_readv(
            pid: crate::unistd::Pid,
            local_iov: &mut [std::io::IoSliceMut<'_>],
            remote_iov: &[RemoteIoVec],
        ) -> crate::Result<usize> {
            if let Some(fake) = fake_system::get() {
                fake.processes().process_vm_readv(pid, local_iov, remote_iov)
            } else {
                real::process_vm_readv(pid, local_iov, remote_iov)
            }
        }
    }
    pub mod mman {
        use super::real::mman as real;
        pub use real::{MapFlags, ProtFlags};
        pub unsafe fn mmap<F: std::os::fd::AsFd>(
            addr: Option<std::num::NonZeroUsize>,
            length: std::num::NonZeroUsize,
            prot: ProtFlags,
            flags: MapFlags,
            f: F,
            offset: libc::off_t,
        ) -> crate::Result<std::ptr::NonNull<std::ffi::c_void>> {
            if let Some(fake) = fake_system::get() {
                fake.processes().mmap(addr, length, prot, flags, f, offset)
            } else {
                real::mmap(addr, length, prot, flags, f, offset)
            }
        }
        pub unsafe fn mmap_anonymous(
            addr: Option<std::num::NonZeroUsize>,
            length: std::num::NonZeroUsize,
            prot: ProtFlags,
            flags: MapFlags,
        ) -> crate::Result<std::ptr::NonNull<std::ffi::c_void>> {
            if let Some(fake) = fake_system::get() {
                fake.processes().mmap_anonymous(addr, length, prot, flags)
            } else {
                real::mmap_anonymous(addr, length, prot, flags)
            }
        }
    }
}

pub mod unistd {
    use super::real::unistd as real;
    pub use real::{sysconf, Pid, SysconfVar};
    pub fn getppid() -> Pid {
        if let Some(fake) = fake_system::get() {
            fake.processes().getppid()
        } else {
            real::getppid()
        }
    }
}

pub use real::{errno, Error, Result};
