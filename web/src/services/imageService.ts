export class ImageService {
    public async uploadImage({img, colorsNum, cellsNum}: {
        img: File,
        colorsNum: string,
        cellsNum: string
    }): Promise<Blob> {
        const formData = new FormData();
        formData.append("file", img);
        formData.append("n_colors", colorsNum);
        formData.append("n_cells_in_width", cellsNum);

        const res = await fetch("/api/upload", {
            method: "POST",
            body: formData
        })
        if (!res.ok) {
            const message = await res.text()
            throw new Error(`Request failed. Status code: ${res.status}. Message: ${message}`)
        }
        return res.blob()
    }
}