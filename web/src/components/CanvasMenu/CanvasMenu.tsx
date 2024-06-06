import { useState } from 'react';
import './CanvasMenu.css';
import CanvasSelector from './CanvasSelector';
import { Canvas } from '../../services/imageService';
import SaveButton from './SaveMenu/SaveButton';
import ClearHistoryButton from './ClearHistoryButton';
import PrintCanvasButton from './PrintCanvasButton';

interface CanvasMenuProps {
    onCanvasSelected: (canvas: Canvas) => void;
    canvas: Canvas;
    showOverlay: (onClose: () => void) => void;
    hideOverlay: () => void;
}

export default function CanvasMenu({
    onCanvasSelected,
    canvas,
    hideOverlay,
    showOverlay,
}: CanvasMenuProps) {
    const [canvasNames, setCanvasNames] = useState<string[]>([]);

    return (
        <div className='canvas-menu-container'>
            <CanvasSelector
                onCanvasSelected={onCanvasSelected}
                canvasNames={canvasNames}
                setCanvasNames={setCanvasNames}
            />
            {canvasNames.length ? (
                <ClearHistoryButton setCanvasNames={setCanvasNames} />
            ) : undefined}
            {canvas.embroidery.length ? (
                <>
                    <SaveButton
                        canvas={canvas}
                        showOverlay={showOverlay}
                        hideOverlay={hideOverlay}
                        canvasNames={canvasNames}
                        setCanvasNames={setCanvasNames}
                    />
                    <PrintCanvasButton />
                </>
            ) : undefined}
        </div>
    );
}
