import {useEffect, useState} from "react";
import {Canvas} from "../../services/imageService";
import CanvasService from "../../services/canvasService";

interface CanvasSelectorProps {
    onCanvasSelected: (canvas: Canvas) => void;
}

export default function CanvasSelector({onCanvasSelected}: CanvasSelectorProps) {
    let [canvases, setCanvases] = useState<string[]>([]);

    async function getSavedCanvases() {
        const canvasService = new CanvasService();
        const canvasNames = await canvasService.getCanvasNames()
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
        const canvasService = new CanvasService();
        const {id, ...canvas} = await canvasService.getCanvasByName(chosenOption);
        onCanvasSelected(canvas)

    }

    return (<select onClick={handleOnClick} onChange={handleOnChange}>
        <option value={""}>Please choose saved canvas</option>
        {canvases.map((option, index) => (<option key={index} value={option}>{option}</option>))}
    </select>);
}