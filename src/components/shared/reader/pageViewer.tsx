import { useState, useEffect, useRef } from "react";
import ReactMarkdown from "react-markdown";
import { useConfigStore } from "../../../stores/configStore";
import { MangaDetail } from "../../../types/ExtensionData";
import { MangaManager } from "../../../utils/MangaManager";
import { getB64 } from "../../../utils/common";
import { fixBook } from "../../../utils/fixBook";

export default function PageViewer() {
    const { config, setPageRoute } = useConfigStore();
    const [page, setPage] = useState(0)
    const [chapterIndex, setChapterIndex] = useState(0)
    const [pages, setPages] = useState<string[]>([]);
    const [mangaDetail, setMangaDetail] = useState({} as MangaDetail);
    const mangaChapter = config.pageRoutes[config.currentPage].state;
    const manga = mangaChapter.manga;

    const touchStartX = useRef(0);
    const touchEndX = useRef(0);
    const SWIPE_THRESHOLD = 50;

    const handleTouchStart = (e: React.TouchEvent) => {
        touchStartX.current = e.changedTouches[0].screenX;
    };

    const handleTouchEnd = (e: React.TouchEvent) => {
        touchEndX.current = e.changedTouches[0].screenX;
        handleSwipe();
    };

    const handleSwipe = () => {
        const difference = touchStartX.current - touchEndX.current;

        if (difference > SWIPE_THRESHOLD) {
            handleNextPage();
        }
        else if (difference < -SWIPE_THRESHOLD) {
            handlePrevPage();
        }
    };

    useEffect(() => {
        const getDetail = async () => {
            if (!config.installedSources || config.installedSources.length === 0) return;

            let detail: MangaDetail;

            const fixedBook = await fixBook(manga, config);
            if (fixedBook.getDetail) {
                detail = await fixedBook.getDetail(fixedBook.link);
            } else {
                detail = fixedBook;
            }
            setMangaDetail(detail);

        };
        getDetail();
    }, [mangaChapter, config]);

    useEffect(() => {
        console.log("mangaDetail:", mangaDetail);

        const index = mangaDetail?.chapters?.findIndex(
            (chapter) => chapter.name === mangaChapter.chapter.name
        ) ?? 0;

        console.log(index, mangaChapter.chapter.name);
        setChapterIndex(index);
    }, [mangaDetail]);

    useEffect(() => {
        const getPages = async () => {
            const source = config.installedSources.find(
                source => source.source.name === mangaChapter.manga.source
            );
            if (!source) return;

            const pageList = await source.getPageList(mangaChapter.chapter.url);
            setPages(pageList);
            setPage(0);
        };

        getPages();
    }, [mangaChapter, config.installedSources]);

    const handleNextPage = async () => {
        setPage(page + 2)
    };

    const handlePrevPage = () => {
        if (page - 2 >= 0) {
            setPage(page - 2);
        }
    };

    return (
        <div
            className="w-full h-full overflow-hidden flex flex-col"
            onTouchStart={handleTouchStart}
            onTouchEnd={handleTouchEnd}
        >

            <div className="flex-1 flex gap-2 sm:gap-4 p-4 sm:p-8 items-center justify-center overflow-hidden">
                {config.layout.doublePanel ? (
                    <>
                        {pages[page] && (
                            <img
                                src={pages[page]}
                                className="max-h-full max-w-1/2 object-contain cursor-pointer rounded"
                                onClick={handleNextPage}
                                alt="Current page"
                            />
                        )}

                        {pages[page + 1] ? (
                            <img
                                src={pages[page + 1]}
                                className="max-h-full max-w-1/2 object-contain cursor-pointer rounded"
                                onClick={handlePrevPage}
                                alt="Next page"
                            />
                        ) : (
                            <div className="max-h-full w-full h-full max-w-1/2 flex items-center justify-center rounded bg-surface text-primary-text">
                                {chapterIndex < mangaDetail.chapters?.length + 1
                                    ? `Next: ${mangaDetail.chapters?.[chapterIndex - 1]?.name}`
                                    : "No more chapters"}
                            </div>
                        )}
                    </>
                ) : (
                    pages[page] && (
                        <img
                            src={pages[page]}
                            className="max-h-full max-w-full object-contain cursor-pointer rounded"
                            onClick={handleNextPage}
                            alt="Current page"
                        />
                    )
                )}

            </div>
        </div>
    );
}