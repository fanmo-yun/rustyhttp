#[derive(Debug)]
pub enum FileType {
    Normal { filename: String },
    Dir { filename: String }
}