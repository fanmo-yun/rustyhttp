use std::{future::Future, path::Path, pin::Pin};
use tokio::fs;

use super::file_type::FileType;

#[derive(Debug)]
pub struct Utils;

impl Default for Utils {
    fn default() -> Self {
        Self::new()
    }
}

impl Utils {
    pub fn new() -> Self {
        Self
    }

    pub async fn read_directories_and_files<T: AsRef<Path> + Send + 'static>(
        &self,
        path: T,
    ) -> tokio::io::Result<Vec<FileType>> {
        let mut file_list = Vec::new();
        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                file_list.push(FileType::Dir { filename: path.display().to_string() });
                let subdir_files = self.read_directories(path).await?;
                file_list.extend(subdir_files);
            } else {
                file_list.push(FileType::Normal { filename: path.display().to_string() });
            }
        }
        Ok(file_list)
    }

    fn read_directories<T: AsRef<Path> + Send + 'static>(
        &self,
        path: T,
    ) -> Pin<Box<dyn Future<Output = tokio::io::Result<Vec<FileType>>> + Send + '_>> {
        Box::pin(async move { self.read_directories_and_files(path).await })
    }
}
