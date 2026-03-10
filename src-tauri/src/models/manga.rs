use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manga {
    author: String,
    chapters: Vec<Chapter>,
    description: String,
    genre: Vec<&String>,
    imageUrl: String, // preferably we would do local image saving for chapters that are in library
    status: MangaStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    name: String,
    url: String,
    scanlator: String,
    dateUpload: String, // its all a string isnt it?
} // always   been        a string.

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum MangaStatus {
    Ongoing = 0,
    Completed = 1,
    Hiatus = 2,
    Cancelled = 3,
}

/* export interface MangaDetail {
    author: string;
    description: string;
    genre: string[];
    status: 0 | 1 | 2 | 3; // ongoing | completed | hiatus | cancelled
    chapters: Chapter[];
    imageUrl?: string;
}
 */
// {name: "Ch.43 Leeep!", url: "22bfd5d8-2d31-4b67-a842-d5aea615171c", scanlator: "MangaPlus", dateUpload: "1767539599000"}

// [Log] Object (BookDetails.tsx, line 28)

// author: "Shindou Masaoki"

// chapters: [Object, Object, Object, Object, Object, Object] (6)

// description: "Ruri faces the usual high school issues: pushy classmates, annoying teachers, and... waking up with dragon horns?! Just after starting hig…"

// genre: ["Award Winning", "School Life", "Monster Girls", "Slice of Life", "Supernatural", "shounen"] (6)

// imageUrl: "https://uploads.mangadex.org/covers/141609b6-cf86-4266-904c-6648f389cdc9/216d1ce9-2195-4ad3-9502-be95b06a3502.jpg"

// status: 0
