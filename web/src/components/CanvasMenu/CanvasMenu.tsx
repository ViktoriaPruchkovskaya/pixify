import CanvasSelector from './CanvasSelector';
import { Canvas } from '../../services/imageService';
import SaveButton from './SaveMenu/SaveButton';

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
    return (
        <div
            style={{
                display: 'flex',
                // justifyContent: 'space-between',
                gap: '15px',
            }}
        >
            <CanvasSelector onCanvasSelected={onCanvasSelected} />
            {canvas.embroidery.length ? (
                <SaveButton
                    canvas={canvas}
                    showOverlay={showOverlay}
                    hideOverlay={hideOverlay}
                />
            ) : undefined}
        </div>
    );
}
