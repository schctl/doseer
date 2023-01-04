//! Path wrapper.

use std::ffi::OsStr;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PathWrap {
    inner: Arc<Path>,
}

impl PathWrap {
    pub fn from_path<T>(path: T) -> io::Result<Self>
    where
        T: AsRef<Path>,
    {
        let path: Rc<Path> = path.as_ref().into();
        let raw = Rc::into_raw(path);
        let path = unsafe { Arc::from_raw(raw) };

        Ok(Self { inner: path })
    }

    #[inline]
    pub fn display(&self) -> &OsStr {
        self.components().next_back().unwrap().as_os_str()
    }
}

impl Deref for PathWrap {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<Path> for PathWrap {
    #[inline]
    fn as_ref(&self) -> &Path {
        self
    }
}
