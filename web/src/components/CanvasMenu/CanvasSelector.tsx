import { useEffect } from 'react';
import { Canvas } from '../../services/imageService';
import CanvasService from '../../services/canvasService';

interface CanvasSelectorProps {
    onCanvasSelected: (canvas: Canvas) => void;
    setCanvasNames: (canvasNames: string[]) => void;
    canvasNames: string[];
}

export default function CanvasSelector({
    onCanvasSelected,
    setCanvasNames,
    canvasNames,
}: CanvasSelectorProps) {
    useEffect(() => {
        (async function () {
            const canvasService = new CanvasService();
            const canvasNames = await canvasService.getCanvasNames();
            setCanvasNames(canvasNames);
        })();
    }, []);

    const handleOnChange = async (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => {
        const chosenOption: string = event.target.value;
        if (!chosenOption) {
            return;
        }
        const canvasService = new CanvasService();
        const { id: _id, ...canvas } =
            await canvasService.getCanvasByName(chosenOption);
        onCanvasSelected(canvas);
    };

    return (
        <select onChange={handleOnChange}>
            <option value={''}>Please choose saved canvas</option>
            {canvasNames.map((option, index) => (
                <option key={index} value={option}>
                    {option}
                </option>
            ))}
        </select>
    );
}
