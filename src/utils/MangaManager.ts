import { invoke } from "@tauri-apps/api/core";

export interface ChapterMeta {
    name: string;
    pages: number[];
}

export interface MangaMeta {
    title: string;
    cover: string | null;
    chapters: ChapterMeta[];
}

export const MangaManager = {
    /**
     * makes new manga including metadata.
     */
    async setup(title: string, description: string, genres: string[]) {
        return await invoke<void>("setup_manga", { mangaTitle: title, mangaDescription: description, mangaGenres: genres });
    },

    /**
     * replace with direct access instead?
     */
    async saveCover(title: string, base64Data: string) {
        return await invoke<void>("save_cover", { mangaTitle: title, data: base64Data });
    },

    /**
     * gets metadata for manga .
     */
    async getMetadata(title: string): Promise<MangaMeta> {
        return await invoke<MangaMeta>("get_meta", { mangaTitle: title });
    },

    /**
     * replaced with direct access again....
     */
    async savePage(title: string, chapter: string, pageNum: number, data: string) {
        return await invoke<void>("save_page", {
            mangaTitle: title,
            chapterName: chapter,
            pageNumber: pageNum,
            data
        });
    }
};