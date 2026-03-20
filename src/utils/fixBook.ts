import { AppConfig, useConfigStore } from '../stores/configStore';
export async function fixBook(book: any, config: AppConfig) {
    const bookSource = book.source?.toLowerCase();
    if (!bookSource) return { ...book };

    const source = config.installedSources.find(
        s => s.source.name.toLowerCase() === bookSource
    );

    if (!source) return { ...book };
    console.log(book)
    return {
        ...book,
        source: source.source.name,
        getDetail: source.getDetail.bind(source)
    };
}