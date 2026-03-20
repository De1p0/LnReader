export const getB64 = async (url: string): Promise<string> => {
    const data = await fetch(url);
    const blob = await data.blob();

    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(blob);
        reader.onloadend = () => {
            const result = reader.result as string;

            const adata = result.split(',')[1];

            resolve(adata);
        };
        reader.onerror = reject;
    });
};