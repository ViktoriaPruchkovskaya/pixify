import * as React from 'react';
import { Canvas, ImageService } from '../../services/imageService';

interface UploadImageFormProps {
    onCanvasReceived: (canvas: Canvas) => void;
}

export default function ImageForm({ onCanvasReceived }: UploadImageFormProps) {
    const handleForm = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const { colorsNum, cellsNum, image } = event.target as HTMLFormElement;
        (async () => {
            const canvas = await new ImageService().uploadImage({
                img: image.files.item(0),
                colorsNum: colorsNum.value,
                cellsNum: cellsNum.value,
            });
            onCanvasReceived(canvas);
        })();
    };
    return (
        <form onSubmit={handleForm}>
            <label htmlFor='image'>Upload Image</label>
            <input type='file' id='image' name='image' />
            <label htmlFor='colorsNum'>Number of colors</label>
            <input
                type='text'
                id='colorsNum'
                name='colorsNum'
                defaultValue='15'
            />
            <label htmlFor='cellsNum'>Number of cells in width</label>
            <input
                type='text'
                id='cellsNum'
                name='cellsNum'
                defaultValue='30'
            />
            <button type='submit'>Submit</button>
        </form>
    );
}
