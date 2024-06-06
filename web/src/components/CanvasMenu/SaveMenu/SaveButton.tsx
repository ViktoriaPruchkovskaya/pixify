import { useState } from 'react';
import { Canvas } from '../../../services/imageService';
import SaveMenu from './SaveMenu';

interface SaveButtonProps {
    canvas: Canvas;
    showOverlay: (onClose: () => void) => void;
    hideOverlay: () => void;
    setCanvasNames: (canvasNames: string[]) => void;
    canvasNames: string[];
}

export default function SaveButton({
    canvas,
    showOverlay,
    hideOverlay,
    setCanvasNames,
    canvasNames,
}: SaveButtonProps) {
    const [isSaveMenuShowed, setIsSaveMenuShowed] = useState(false);
    const handleOnClick = () => {
        setIsSaveMenuShowed(true);
        showOverlay(() => {
            setIsSaveMenuShowed(false);
        });
    };

    return (
        <>
            <button onClick={handleOnClick} style={{ width: '10%' }}>
                Save
            </button>
            {isSaveMenuShowed && (
                <SaveMenu
                    canvas={canvas}
                    setIsSaveMenuShowed={setIsSaveMenuShowed}
                    hideOverlay={hideOverlay}
                    canvasNames={canvasNames}
                    setCanvasNames={setCanvasNames}
                />
            )}
        </>
    );
}
