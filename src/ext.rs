use std::path::{Path, PathBuf};

use crate::{error::MetadataError, from_objc::FromObjC, mds::query_file_attr};

pub trait PathBufExt {
    /// Query a file attribute using the Metadata Service
    ///
    /// # Example
    /// ```
    /// let path = PathBuf::from("audio.mp3");
    /// let attr = path.attr::<String>("kMDItemContentType");
    /// ```
    fn attr<T: FromObjC>(&self, attr_name: &str) -> Result<T, MetadataError>
    where
        Self: AsRef<Path>,
    {
        query_file_attr(self, attr_name)
    }

    /// Returns the `kMDItemKind` attribute of the file. (e.g. "Plain Text Document" or "JPEG image")
    fn kind(&self) -> Option<String>
    where
        Self: AsRef<Path>,
    {
        query_file_attr(self, "kMDItemKind").ok()
    }

    /// Returns the `kMDItemContentType` attribute of the file. (e.g. "public.plain-text")
    fn content_type(&self) -> Option<String>
    where
        Self: AsRef<Path>,
    {
        query_file_attr(self, "kMDItemContentType").ok()
    }

    /// Returns the `kMDItemContentTypeTree` attribute of the file.
    ///
    /// (e.g. `[public.jpeg, public.image, public.data, public.item, public.content]`)
    fn content_type_tree(&self) -> Option<Vec<String>>
    where
        Self: AsRef<Path>,
    {
        query_file_attr(self, "kMDItemContentTypeTree").ok()
    }

    /// Returns the `kMDItemWhereFroms` attribute of the file.
    ///
    /// (e.g. `["https://example.com"]`)
    fn where_from(&self) -> Option<Vec<String>>
    where
        Self: AsRef<Path>,
    {
        query_file_attr(self, "kMDItemWhereFroms").ok()
    }
}

impl PathBufExt for PathBuf {}
