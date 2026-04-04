import { invoke } from "@tauri-apps/api/core";

export interface ChapterMeta {
    name: string;
    read: boolean;
}

export interface MangaMeta {
    title: string;
    description: string;
    genres: string[];
    cover_url: string | null;
    chapters: ChapterMeta[];
}

export const MangaManager = {
    async add(title: string, description: string, genres: string[], coverUrl?: string) {
        return await invoke<void>("add_manga", {
            title,
            description,
            genres,
            coverUrl: coverUrl ?? null,
        });
    },

    async addChapter(mangaTitle: string, chapterName: string) {
        return await invoke<void>("add_chapter", { mangaTitle, chapterName });
    },

    async setChapterRead(mangaTitle: string, chapterName: string, read: boolean) {
        return await invoke<void>("set_chapter_read", { mangaTitle, chapterName, read });
    },

    async get(mangaTitle: string): Promise<MangaMeta> {
        return await invoke<MangaMeta>("get_manga", { mangaTitle });
    },

    async list(): Promise<MangaMeta[]> {
        return await invoke<MangaMeta[]>("list_manga");
    },

    async remove(mangaTitle: string) {
        return await invoke<void>("remove_manga", { mangaTitle });
    },
    async saveCover(title: string, base64Data: string) {
        return await invoke<void>("save_cover", { mangaTitle: title, data: base64Data });
    },

};