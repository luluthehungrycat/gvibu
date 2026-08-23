//! Allocation-free, typed contracts layered over raw VIBIX syscalls.

use crate::sys;

/// A decoded VIBIX syscall failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SysError {
    /// A negative errno value returned by the kernel.
    Errno(u16),
    /// The syscall-specific `u64::MAX` failure sentinel.
    FailureSentinel,
    /// A non-empty write made no progress.
    WriteZero,
    /// The kernel reported more bytes than were requested.
    InvalidWriteCount,
}

pub type SysResult<T> = Result<T, SysError>;

/// Convert a raw VIBIX return value before using it as a successful value.
#[inline]
pub const fn decode_raw(raw: u64) -> SysResult<u64> {
    if raw == u64::MAX {
        Err(SysError::FailureSentinel)
    } else if (raw as i64) < 0 {
        Err(SysError::Errno((-(raw as i64)) as u16))
    } else {
        Ok(raw)
    }
}

/// A non-negative VIBIX file descriptor.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fd(i32);

impl Fd {
    pub const STDIN: Self = Self(0);
    pub const STDOUT: Self = Self(1);
    pub const STDERR: Self = Self(2);

    pub const fn from_raw(raw: i32) -> Option<Self> {
        if raw < 0 {
            None
        } else {
            Some(Self(raw))
        }
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

/// Issue one checked compatibility write through syscall 1.
#[inline]
pub fn write(fd: Fd, bytes: &[u8]) -> SysResult<usize> {
    if bytes.is_empty() {
        return Ok(0);
    }

    let raw = sys::sys_write(fd.raw(), bytes.as_ptr(), bytes.len());
    let count = decode_raw(raw)? as usize;
    if count > bytes.len() {
        Err(SysError::InvalidWriteCount)
    } else {
        Ok(count)
    }
}

/// Write all bytes, retrying short writes and rejecting zero progress.
#[inline]
pub fn write_all(fd: Fd, bytes: &[u8]) -> SysResult<()> {
    write_all_with(fd, bytes, write)
}

/// Testable write-all implementation over an injected single-write function.
#[inline]
pub fn write_all_with<F>(fd: Fd, bytes: &[u8], mut write_once: F) -> SysResult<()>
where
    F: FnMut(Fd, &[u8]) -> SysResult<usize>,
{
    let mut offset = 0;
    while offset < bytes.len() {
        let count = write_once(fd, &bytes[offset..])?;
        if count == 0 {
            return Err(SysError::WriteZero);
        }
        let remaining = bytes.len() - offset;
        if count > remaining {
            return Err(SysError::InvalidWriteCount);
        }
        offset += count;
    }
    Ok(())
}

#[inline]
pub fn write_stdout(bytes: &[u8]) -> SysResult<()> {
    write_all(Fd::STDOUT, bytes)
}

#[inline]
pub fn write_stderr(bytes: &[u8]) -> SysResult<()> {
    write_all(Fd::STDERR, bytes)
}

/// Decode a raw program-break result without issuing a syscall.
#[inline]
pub const fn decode_brk(raw: u64) -> SysResult<usize> {
    match decode_raw(raw) {
        Ok(value) => Ok(value as usize),
        Err(error) => Err(error),
    }
}

/// Query or update the process break without exposing an unchecked pointer.
#[inline]
pub fn brk(addr: usize) -> SysResult<usize> {
    decode_brk(sys::sys_brk(addr))
}

#[cfg(test)]
mod tests {
    use super::{brk, decode_brk, decode_raw, write_all_with, Fd, SysError};

    #[test]
    fn decodes_negative_errno() {
        assert_eq!(decode_raw((-9i64) as u64), Err(SysError::Errno(9)));
    }

    #[test]
    fn decodes_failure_sentinel() {
        assert_eq!(decode_raw(u64::MAX), Err(SysError::FailureSentinel));
    }

    #[test]
    fn preserves_non_negative_values() {
        assert_eq!(decode_raw(13), Ok(13));
    }

    #[test]
    fn rejects_negative_descriptors() {
        assert_eq!(Fd::from_raw(-1), None);
        assert_eq!(Fd::from_raw(3).map(Fd::raw), Some(3));
    }

    #[test]
    fn standard_descriptors_are_stable() {
        assert_eq!(Fd::STDIN.raw(), 0);
        assert_eq!(Fd::STDOUT.raw(), 1);
        assert_eq!(Fd::STDERR.raw(), 2);
    }

    #[test]
    fn routes_stdout_and_stderr_without_syscalls() {
        let mut seen = [0; 2];
        write_all_with(Fd::STDOUT, b"out", |fd, bytes| {
            seen[0] = fd.raw();
            assert_eq!(bytes, b"out");
            Ok(bytes.len())
        })
        .unwrap();
        write_all_with(Fd::STDERR, b"err", |fd, bytes| {
            seen[1] = fd.raw();
            assert_eq!(bytes, b"err");
            Ok(bytes.len())
        })
        .unwrap();
        assert_eq!(seen, [1, 2]);
    }

    #[test]
    fn retries_short_writes() {
        let mut calls = 0;
        write_all_with(Fd::STDOUT, b"abcd", |_fd, bytes| {
            calls += 1;
            Ok(bytes.len().min(2))
        })
        .unwrap();
        assert_eq!(calls, 2);
    }

    #[test]
    fn rejects_zero_progress() {
        assert_eq!(
            write_all_with(Fd::STDOUT, b"x", |_fd, _bytes| Ok(0)),
            Err(SysError::WriteZero)
        );
    }

    #[test]
    fn propagates_invalid_descriptor_errno() {
        assert_eq!(
            write_all_with(Fd::STDOUT, b"x", |_fd, _bytes| {
                decode_raw((-9i64) as u64).map(|_| 1)
            }),
            Err(SysError::Errno(9))
        );
    }
    #[test]
    fn brk_failure_is_not_a_pointer() {
        assert_eq!(decode_brk(u64::MAX), Err(SysError::FailureSentinel));
        let _ = brk as fn(usize) -> Result<usize, SysError>;
    }
}
