use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::command;

#[derive(Clone)]
pub struct Manga {
    pub title: String,
}

#[derive(Clone)]
pub struct Chapter {
    pub manga_title: String,
    pub name: String,
}

#[derive(Clone, Copy)]
pub struct Page {
    pub number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MangaMeta {
    pub title: String,
    pub cover: Option<String>,
    pub chapters: Vec<ChapterMeta>,
    pub description: String,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterMeta {
    pub name: String,
    pub pages: Vec<u32>,
}

pub struct MangaManager {
    pub base_path: PathBuf,
}

impl MangaManager {
    pub fn new(path: PathBuf) -> Self {
        println!("Manga manager initialized at {:?}", path);
        Self { base_path: path }
    }

    fn manga_dir(&self, manga: &Manga) -> PathBuf {
        let safe_title = self.sanitize_name(&manga.title);
        self.base_path.join("manga").join(safe_title)
    }

    fn chapter_dir(&self, chapter: &Chapter) -> PathBuf {
        let safe_manga = self.sanitize_name(&chapter.manga_title);
        let safe_chapter = self.sanitize_name(&chapter.name);

        self.base_path
            .join("manga")
            .join(safe_manga)
            .join(safe_chapter)
    }

    fn page_path(&self, chapter: &Chapter, page: Page) -> PathBuf {
        self.chapter_dir(chapter)
            .join(format!("{}.png", page.number))
    }

    fn cover_path(&self, manga: &Manga) -> PathBuf {
        self.manga_dir(manga).join("cover.png")
    }

    fn meta_path(&self, manga: &Manga) -> PathBuf {
        self.manga_dir(manga).join("manga.json")
    }

    fn save_base64_image(&self, path: &Path, data: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("dir error: {}", e))?;
        }
        let bytes = STANDARD
            .decode(data)
            .map_err(|e| format!("decode error: {}", e))?;
        fs::write(path, bytes).map_err(|e| format!("write error: {}", e))?;
        Ok(())
    }

    fn load_meta(&self, manga: &Manga) -> Result<MangaMeta, String> {
        let data = fs::read_to_string(self.meta_path(manga)).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }

    fn save_meta(&self, manga: &Manga, meta: &MangaMeta) -> Result<(), String> {
        let json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
        fs::write(self.meta_path(manga), json).map_err(|e| e.to_string())
    }

    fn sanitize_name(&self, name: &str) -> String {
        name.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
    }
}

#[command]
pub async fn setup_manga(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    manga_description: String,
    manga_genres: Vec<String>,
) -> Result<(), String> {
    let manga = Manga {
        title: manga_title.clone(),
    };
    let dir = manager.manga_dir(&manga);

    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    // let meta_path = manager.meta_path(&manga);
    // if meta_path.exists() {
    //     return Err(format!(
    //         "Manga already exists so i fucking deleted it {:?}",
    //         meta_path
    //     )
    //     .into());
    // }

    println!("Creating manga: {}", manga_title);

    let meta = MangaMeta {
        title: manga.title.clone(),
        cover: None,
        chapters: vec![],
        description: manga_description,
        genres: manga_genres,
    };

    manager
        .save_meta(&manga, &meta)
        .map_err(|e| format!("Creating meta data failed: {}", e))?;

    Ok(())
}

#[command]
pub async fn save_page(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    chapter_name: String,
    page_number: u32,
    data: String,
) -> Result<(), String> {
    let chapter = Chapter {
        manga_title: manga_title.clone(),
        name: chapter_name,
    };
    let page = Page {
        number: page_number,
    };
    let manga = Manga { title: manga_title };

    manager.save_base64_image(&manager.page_path(&chapter, page), &data)?;

    let meta = manager.load_meta(&manga)?;
    manager.save_meta(&manga, &meta)?;
    Ok(())
}

#[command]
pub async fn save_cover(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
    data: String,
) -> Result<(), String> {
    let manga = Manga { title: manga_title };
    println!("Saving cover at path: {:?}", manager.cover_path(&manga));
    manager.save_base64_image(&manager.cover_path(&manga), &data)?;
    let mut meta = manager.load_meta(&manga)?;
    meta.cover = Some("cover.png".into());
    manager.save_meta(&manga, &meta)?;
    Ok(())
}

#[command]
pub async fn get_meta(
    manager: tauri::State<'_, MangaManager>,
    manga_title: String,
) -> Result<MangaMeta, String> {
    let manga = Manga { title: manga_title };
    manager.load_meta(&manga)
}
