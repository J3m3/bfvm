use std::fmt;

#[derive(Debug)]
pub struct JitCompileError {
    message: String,
}

impl JitCompileError {
    // When target_arch and target_os are not matched, this associated function is never used
    #[allow(dead_code)]
    fn with_ip(ip: usize, message: &str) -> Self {
        JitCompileError {
            message: format!("{message} [IP:{ip}]"),
        }
    }

    fn new(message: &str) -> Self {
        JitCompileError {
            message: format!("{message}"),
        }
    }
}

impl fmt::Display for JitCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JIT COMPILE ERROR: {}", self.message)
    }
}

impl From<std::io::Error> for JitCompileError {
    fn from(value: std::io::Error) -> Self {
        JitCompileError {
            message: format!("{value}"),
        }
    }
}

impl From<JitCompileError> for std::io::Error {
    fn from(value: JitCompileError) -> Self {
        value.into()
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub mod aarch64;
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub use aarch64::jit_compile;

#[cfg(not(any(all(target_arch = "aarch64", target_os = "linux"))))]
pub fn jit_compile(
    _input: &str,
    _memory: &mut crate::Memory,
) -> Result<memmap2::Mmap, JitCompileError> {
    Err(JitCompileError::new(
        "JIT compiler is not supported on this architecture with OS",
    ))
}
