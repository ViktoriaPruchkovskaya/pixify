import {useEffect, useState} from "react";
import {StorageService} from "../../services/storageService";
import {Canvas} from "../../services/imageService";

interface CanvasMenuProps {
    onCanvasChange: (canvas: Canvas) => void;
}

export default function CanvasMenu({onCanvasChange}: CanvasMenuProps) {
    let [canvases, setCanvases] = useState([]);

    async function getSavedCanvases() {
        const storageService = await StorageService.getInstance();
        const canvasNames = await storageService.getCanvasNames()
        setCanvases(canvasNames)
    }

    useEffect(() => {
        (async function () {
            await getSavedCanvases()
        })()
    }, [])

    const handleOnClick = async () => {
        await getSavedCanvases()
    }

    const handleOnChange = async (event: React.ChangeEvent<HTMLSelectElement>) => {
        const chosenOption: string = event.target.value;
        if (!chosenOption) {
            return
        }
        const storageService = await StorageService.getInstance();
        const {id, ...canvas} = await storageService.getCanvasByName(chosenOption);
        onCanvasChange(canvas)

    }

    return (<select onClick={handleOnClick} onChange={handleOnChange}>
        <option value={""}>Please choose saved canvas</option>
        {canvases.map((option, index) => (<option key={index} value={option}>{option}</option>))}
    </select>);
}