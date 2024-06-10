import { useState } from 'react';
import './App.css';
import ImageForm from './components/ImageUpload/ImageForm';
import EmbroideryCanvas from './components/EmbroideryCanvas/EmbroideryCanvas';
import { Canvas } from './services/imageService';
import ColorSelector from './components/ColorSelector/ColorSelector';
import { useColorContextMenu } from './hooks/useColorContextMenu';
import { useSelectedCell } from './hooks/useSelectedCell';
import ThreadsPalette from './components/ThreadsPalette/ThreadsPalette';
import CanvasMenu from './components/CanvasMenu/CanvasMenu';
import { useOverlay } from './hooks/useOverlay';

export default function App() {
    const [canvas, setCanvas] = useState<Canvas>({
        embroidery: [],
        palette: [
            { identifier: '00', color: { name: '', rgb: [] }, nStitches: 0 },
        ],
    });

    const { showOverlay, hideOverlay, isOverlayRendered, onClose } =
        useOverlay();
    const { setSelectedCellPosition, resetCellPosition, selectedCellPosition } =
        useSelectedCell();
    const { canvasUpdater, setCanvasUpdater, showMenu, colorSelectorStyle } =
        useColorContextMenu(showOverlay, resetCellPosition);

    return (
        <div>
            {isOverlayRendered && (
                <div
                    className='overlay'
                    onClick={() => {
                        onClose && onClose();
                        hideOverlay();
                    }}
                />
            )}
            <ColorSelector
                dynamicStyles={colorSelectorStyle}
                palette={canvas.palette}
                updateCanvas={canvasUpdater}
            />
            <div className='header-container'>
                <ImageForm onCanvasReceived={setCanvas} />
                <CanvasMenu
                    onCanvasSelected={setCanvas}
                    canvas={canvas}
                    showOverlay={showOverlay}
                    hideOverlay={hideOverlay}
                />
            </div>

            {canvas?.embroidery.length ? (
                <div className='canvas-container'>
                    <EmbroideryCanvas
                        style={
                            isOverlayRendered ? { pointerEvents: 'none' } : {}
                        }
                        canvas={canvas}
                        onCanvasChange={setCanvas}
                        changeCanvasUpdater={setCanvasUpdater}
                        showMenu={showMenu}
                        setSelectedCellPosition={setSelectedCellPosition}
                        selectedCellPosition={selectedCellPosition}
                    />
                    <div className='page-break'></div>
                    <ThreadsPalette palette={canvas.palette} />
                </div>
            ) : undefined}
        </div>
    );
}
