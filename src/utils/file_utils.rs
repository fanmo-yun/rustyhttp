use std::path::Path;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

#[derive(Debug)]
pub struct Utils {
    dir: &'static Path,
}

impl Utils {
    pub fn new(dir: &'static Path) -> Self {
        Self { dir }
    }

    // pub async fn read_directories_and_files<T: AsRef<Path> + Send + 'static>(
    //     &self,
    //     path: T,
    // ) -> tokio::io::Result<Vec<FileType>> {
    //     let mut file_list = Vec::new();
    //     let mut entries = fs::read_dir(path).await?;

    //     while let Some(entry) = entries.next_entry().await? {
    //         let path = entry.path();

    //         if path.is_dir() {
    //             file_list.push(FileType::Dir { filename: path.display().to_string() });
    //             let subdir_files = self.read_directories(path).await?;
    //             file_list.extend(subdir_files);
    //         } else {
    //             file_list.push(FileType::Normal { filename: path.display().to_string() });
    //         }
    //     }
    //     Ok(file_list)
    // }

    // fn read_directories<T: AsRef<Path> + Send + 'static>(
    //     &self,
    //     path: T,
    // ) -> Pin<Box<dyn Future<Output = tokio::io::Result<Vec<FileType>>> + Send + '_>> {
    //     Box::pin(async move { self.read_directories_and_files(path).await })
    // }

    pub async fn read_file<T: AsRef<Path>>(&self, file_path: T) -> tokio::io::Result<Vec<u8>> {
        let full_path = self.dir.join(file_path);
        let file = File::open(full_path).await?;
        let mut reader = BufReader::new(file);
        let mut context = Vec::new();
        reader.read_to_end(&mut context).await?;
        Ok(context)
    }
}
