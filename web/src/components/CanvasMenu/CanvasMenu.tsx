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
        <div>
            <CanvasSelector onCanvasSelected={onCanvasSelected} />
            <SaveButton
                canvas={canvas}
                showOverlay={showOverlay}
                hideOverlay={hideOverlay}
            />
        </div>
    );
}
