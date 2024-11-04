// TODO -- Implement ptrace

use libc as real;

pub use real::{
    c_int, c_long, c_ulonglong, c_void, iovec, off_t, pid_t, printf, pthread_kill, pthread_self,
    sigaction, sigemptyset, siginfo_t, signalfd_siginfo, stat, user, user_fpregs_struct,
    user_regs_struct, Elf64_Addr, AT_ENTRY, AT_NULL, AT_PHDR, AT_PHNUM, AT_SYSINFO_EHDR,
    REG_CSGSFS, REG_EFL, REG_R10, REG_R11, REG_R12, REG_R13, REG_R14, REG_R15, REG_R8, REG_R9,
    REG_RAX, REG_RBP, REG_RBX, REG_RCX, REG_RDI, REG_RDX, REG_RIP, REG_RSI, REG_RSP, SA_SIGINFO,
    SIGHUP,
};

pub unsafe fn ptrace<A, B, C>(request: real::c_uint, a: A, b: B, c: C) -> real::c_long {
    if let Some(fake) = fake_system::get() {
        fake.processes().ptrace(request, a, b, c)
    } else {
        real::ptrace(request, a, b, c)
    }
}
