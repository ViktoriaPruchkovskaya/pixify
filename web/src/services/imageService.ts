export interface Canvas {
    embroidery: number[][][];
    palette: PaletteColor[];
}

export interface PaletteColor {
    identifier: string;
    color: { rgb: number[]; name: string };
    nStitches: number;
}

export class ImageService {
    public async uploadImage({
        img,
        colorsNum,
        cellsNum,
    }: {
        img: File;
        colorsNum: string;
        cellsNum: string;
    }): Promise<Canvas> {
        const formData = new FormData();
        formData.append('file', img);
        formData.append('nColors', colorsNum);
        formData.append('nCellsInWidth', cellsNum);

        const res = await fetch('/api/upload', {
            method: 'POST',
            body: formData,
        });
        if (!res.ok) {
            const message = await res.text();
            throw new Error(
                `Request failed. Status code: ${res.status}. Message: ${message}`
            );
        }
        return res.json();
    }
}
