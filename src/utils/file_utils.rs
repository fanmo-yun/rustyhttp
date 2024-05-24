use std::{future::Future, path::Path, pin::Pin};

use tokio::{fs, io};

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
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                println!("Directory: {}", path.display());
                self.read_directories(path).await?;
            } else {
                println!("File: {}", path.display());
            }
        }
        Ok(())
    }

    fn read_directories<T: AsRef<Path> + Send + 'static>(
        &self,
        path: T,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + '_>> {
        Box::pin(async move { self.read_directories_and_files(path).await })
    }
}
