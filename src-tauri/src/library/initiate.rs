use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::{fmt::format, fs, path::PathBuf};
use tauri::command;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MangaMeta {
    pub title: String,
    pub description: String,
    pub genres: Vec<String>,
    pub cover_url: Option<String>,
    pub cover_path: Option<String>,
    pub chapters: Vec<ChapterMeta>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChapterMeta {
    pub name: String,
    pub read: bool,
}

pub struct MangaManager {
    pub base_path: PathBuf,
}

impl MangaManager {
    pub fn new(path: PathBuf) -> Self {
        fs::create_dir_all(&path).ok();
        Self { base_path: path }
    }

    fn manga_dir(&self, manga_title: &str) -> PathBuf {
        let safe = manga_title
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>();

        self.base_path.join(safe)
    }

    fn meta_path(&self, manga_title: &str) -> PathBuf {
        let dir = self.manga_dir(manga_title);
        let name = dir.file_name().unwrap().to_string_lossy();
        dir.join(format!("{}.json", name))
    }

    fn cover_path(&self, manga_title: &str) -> PathBuf {
        let dir = self.manga_dir(manga_title);
        let name = dir.file_name().unwrap().to_string_lossy();
        dir.join(format!("{}.png", name))
    }

    fn load(&self, manga_title: &str) -> Result<MangaMeta, String> {
        let data = fs::read_to_string(self.meta_path(manga_title)).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }

    fn save(&self, meta: &MangaMeta) -> Result<(), String> {
        let json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
        fs::write(self.meta_path(&meta.title), json).map_err(|e| e.to_string())
    }
}

#[command]
pub async fn save_cover(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    data: String,
) -> Result<(), String> {
    let path = manager.cover_path(&manga_title);
    let bytes = STANDARD
        .decode(&data)
        .map_err(|e| format!("decode error: {}", e))?;
    println!("Saving cover at : \"{:?}\"", &path);
    fs::create_dir_all(manager.manga_dir(&manga_title));

    fs::write(&path, bytes).map_err(|e| format!("write error: {}", e))?;

    let mut meta = manager.load(&manga_title)?;
    meta.cover_path = Some(path.to_string_lossy().to_string());
    manager.save(&meta)
}

#[command]
pub async fn add_manga(
    manager: tauri::State<'_, MangaManager>,
    title: String,
    description: String,
    genres: Vec<String>,
    cover_url: Option<String>,
) -> Result<(), String> {
    fs::create_dir_all(manager.manga_dir(&title));

    let meta = MangaMeta {
        title,
        description,
        genres,
        cover_url,
        cover_path: None,
        chapters: vec![],
    };
    manager.save(&meta)
}

#[command]
pub async fn add_chapter(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    chapter_name: String,
) -> Result<(), String> {
    let mut meta = manager.load(&manga_title)?;
    if !meta.chapters.iter().any(|c| c.name == chapter_name) {
        meta.chapters.push(ChapterMeta {
            name: chapter_name,
            read: false,
        });
        manager.save(&meta)?;
    }
    Ok(())
}

#[command]
pub async fn set_chapter_read(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    chapter_name: String,
    read: bool,
) -> Result<(), String> {
    let mut meta = manager.load(&manga_title)?;
    let chapter = meta
        .chapters
        .iter_mut()
        .find(|c| c.name == chapter_name)
        .ok_or("Chapter not found")?;
    chapter.read = read;
    manager.save(&meta)
}

#[command]
pub async fn get_manga(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
) -> Result<MangaMeta, String> {
    manager.load(&manga_title)
}

#[command]
pub async fn list_manga(manager: tauri::State<'_, MangaManager>) -> Result<Vec<MangaMeta>, String> {
    let entries = fs::read_dir(&manager.base_path).map_err(|e| e.to_string())?;
    let mut result = vec![];
    for entry in entries.flatten() {
        if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
            if let Ok(data) = fs::read_to_string(entry.path()) {
                if let Ok(meta) = serde_json::from_str::<MangaMeta>(&data) {
                    result.push(meta);
                }
            }
        }
    }
    Ok(result)
}

#[command]
pub async fn remove_manga(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
) -> Result<(), String> {
    fs::remove_file(manager.meta_path(&manga_title)).map_err(|e| e.to_string())
}
