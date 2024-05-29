import {Canvas} from "../../services/imageService";
import * as React from "react";
import {StorageService} from "../../services/storageService";

interface SaveButtonProps {
    canvas: Canvas;
}

export default function SaveButton({canvas}: SaveButtonProps) {
    const handleOnClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        event.preventDefault();
        (async () => {
            const storageService = await StorageService.getInstance();
            await storageService.setCanvas(canvas);
        })()
    }
    return (<button onClick={handleOnClick}>SAVE</button>)
}