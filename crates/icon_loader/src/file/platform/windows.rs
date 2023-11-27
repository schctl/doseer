//! Windows Shell icon loader.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use moka::sync::Cache;

use widestring::{U16CStr, U16CString};
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::BITMAP;
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::UI::Controls::IImageList;
use windows::Win32::UI::Shell::{
    SHGetFileInfoW, SHGetImageList, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHIL_JUMBO,
};
use windows::Win32::UI::WindowsAndMessaging::{GetIconInfo, HICON, ICONINFO};

use crate::file::{Icon, ImageOrSvg};

/// Icon loader implementation for windows shell.
#[derive(Debug)]
pub struct Loader {
    // TODO: don't cache every single file path, instead cache
    // based on file type, handling special paths like home, etc.
    icon_cache: Cache<PathBuf, Icon>,
}

impl Loader {
    #[inline]
    pub fn new() -> Self {
        Self {
            icon_cache: Cache::builder().max_capacity(128).build(),
        }
    }

    pub fn load(&self, path: &Path) -> Option<Icon> {
        if let Some(icon) = self.icon_cache.get(path) {
            return Some(icon);
        }

        if let Some(icon) = self.load_winicon(path) {
            self.icon_cache.insert(path.to_owned(), icon.clone());
            return Some(icon);
        }

        None
    }

    fn load_winicon(&self, path: &Path) -> Option<Icon> {
        let path_c = U16CString::from_os_str(path).ok()?;

        let image = hicon::convert_to_image(get_icon(&path_c))?;

        Some(Arc::new(ImageOrSvg::Image(
            iced_native::image::Handle::from_pixels(image.width, image.height, image.data),
        )))
    }
}

pub fn get_icon(path: &U16CStr) -> HICON {
    let mut shfi = SHFILEINFOW::default();
    let path = PCWSTR::from_raw(path.as_ptr());

    unsafe {
        SHGetFileInfoW(
            path,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut shfi),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );
    }

    // SHGetFileInfoW only gives us small (32x32) icons, so we access large icons from an image list
    // See https://stackoverflow.com/a/28015423/15842331

    unsafe {
        if let Ok(image_list) = SHGetImageList::<IImageList>(SHIL_JUMBO as i32) {
            if let Ok(large_hicon) = image_list.GetIcon(shfi.iIcon, 0) {
                return large_hicon;
            }
        }
    }

    shfi.hIcon
}

mod hicon {
    use windows::Win32::Graphics::Gdi::{GetBitmapBits, GetObjectW};

    use super::*;

    pub struct Bitmap {
        pub width: u32,
        pub height: u32,
        pub data: Vec<u8>,
    }

    pub fn convert_to_image(hicon: HICON) -> Option<Bitmap> {
        unsafe {
            // convert it into a BITMAP first
            let mut icon_info = ICONINFO::default();
            GetIconInfo(hicon, &mut icon_info).ok()?;

            let mut bitmap = BITMAP::default();
            GetObjectW(
                icon_info.hbmColor,
                std::mem::size_of::<BITMAP>() as i32,
                Some(&mut bitmap as *mut _ as *mut _),
            );

            // extract raw bits
            let size = (bitmap.bmWidthBytes * bitmap.bmHeight) as usize;

            let mut bits = vec![0; size];

            // Thanks! https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975/15

            GetBitmapBits(icon_info.hbmColor, size as i32, bits.as_mut_ptr() as *mut _);

            Some(Bitmap {
                width: bitmap.bmWidth as u32,
                height: bitmap.bmHeight as u32,
                data: bits,
            })
        }
    }
}
