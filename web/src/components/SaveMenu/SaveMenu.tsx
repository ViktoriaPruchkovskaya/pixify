import * as React from "react";
import {Canvas} from "../../services/imageService";
import {StorageService} from "../../services/storageService";

interface SaveMenuProps {
    canvas: Canvas;
    setIsSaveMenuShowed: (isShowed: boolean) => void;
}

export default function SaveMenu({canvas, setIsSaveMenuShowed}: SaveMenuProps) {
    const handleForm = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const {canvasName} = event.target as HTMLFormElement;
        (async () => {
            const storageService = await StorageService.getInstance();
            await storageService.setCanvas(canvas, canvasName.value);
        })()
        setIsSaveMenuShowed(false)
    }

    return (<div style={{
        display: "block",
        position: "fixed",
        width: "400px",
        height: "200px",
        backgroundColor: "white",
        borderRadius: "2px",
        boxShadow: "0 2px 9px rgba(0, 0, 0, 0.6)",
    }}>
        <form onSubmit={handleForm}>
            <label htmlFor="image">Canvas Name</label>
            <input type="text" id="canvasName" name="canvasName" defaultValue="canvas"/>
            <button type="submit">Save</button>
        </form>
    </div>)
}