use std::fs;

// 画像か動画の判定結果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Image,
    Video,
    Unknown,
}

pub fn infer_type(filepath: &str) -> FileType {
    let mut file_type = FileType::Unknown;
    if let Ok(buf) = fs::read(filepath) {
         if let Some(kind) = infer::get(&buf) {
            if kind.mime_type().starts_with("image/") {
                file_type = FileType::Image;
            } else if kind.mime_type().starts_with("video/") {
                file_type = FileType::Video;
            }
        };
    }
    file_type
}