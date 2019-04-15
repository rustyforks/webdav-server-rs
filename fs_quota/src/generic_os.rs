//
// No-op implementations.
//
use std::io;
use std::path::Path;

use crate::{FsQuota, FsType, FqError, Mtab};

pub(crate) fn get_quota(_device: impl AsRef<Path>, _fstype: FsType, _uid: u32) -> Result<FsQuota, FqError> {
    Err(FqError::NoQuota)
}

pub(crate) fn read_mtab() -> io::Result<Vec<Mtab>> {
    Ok(Vec::new())
}

pub(crate) fn statvfs(_path: impl AsRef<Path>) -> Result<FsQuota, FqError> {
    Err(FqError::NoQuota)
}
