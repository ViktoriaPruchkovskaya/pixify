import { CSSProperties } from 'react';
import './EmbroideryCanvas.css';
import EmbroideryRow from './EmbroideryRow';
import { Canvas, PaletteColor } from '../../services/imageService';

interface EmbroideryCanvasProps {
    canvas: Canvas;
    onCanvasChange: (canvas: Canvas) => void;
    changeCanvasUpdater: (cb: (color: number[]) => void) => void;
    showMenu: (xPos: number, yPos: number) => void;
    style?: CSSProperties;
    setSelectedCellPosition: (row: number) => (column: number) => () => void;
    selectedCellPosition?: { row: number; column: number };
}

export default function EmbroideryCanvas({
    canvas,
    onCanvasChange,
    changeCanvasUpdater,
    showMenu,
    style,
    setSelectedCellPosition,
    selectedCellPosition,
}: EmbroideryCanvasProps) {
    const getPalette = () => {
        const palette: Map<string, string> = new Map();
        canvas.palette.forEach(({ identifier, color }) =>
            palette.set(color.rgb.toString(), identifier)
        );
        return palette;
    };

    const updateCanvas = (rowId: number) => {
        const newEmbroidery: number[][][] = [...canvas.embroidery];
        const newPalette: PaletteColor[] = [...canvas.palette];

        const isEqual = (arr1: number[], arr2: number[]): boolean =>
            arr1.every((el, index) => el === arr2[index]);

        return (cellId: number) => {
            return (rgb: number[]) => {
                const oldColor: number[] = newEmbroidery[rowId][cellId];
                newEmbroidery[rowId][cellId] = rgb;

                canvas.embroidery = newEmbroidery;
                canvas.palette = newPalette.reduce(
                    (threads: PaletteColor[], thread: PaletteColor) => {
                        if (isEqual(thread.color.rgb, rgb)) {
                            thread.nStitches += 1;
                        }
                        if (isEqual(thread.color.rgb, oldColor)) {
                            thread.nStitches -= 1;
                        }

                        return thread.nStitches > 0
                            ? [...threads, thread]
                            : threads;
                    },
                    []
                );
                onCanvasChange({ ...canvas });
            };
        };
    };

    return (
        <div className='embroidery-canvas-container' style={style}>
            <table className='canvas-content'>
                <tbody>
                    {canvas.embroidery.map((row, i) => (
                        <EmbroideryRow
                            key={i}
                            row={row}
                            palette={getPalette()}
                            updateCanvas={updateCanvas(i)}
                            changeCanvasUpdater={changeCanvasUpdater}
                            showMenu={showMenu}
                            setSelectedCellPosition={setSelectedCellPosition(i)}
                            selectedCellPosition={
                                selectedCellPosition?.row === i
                                    ? selectedCellPosition
                                    : undefined
                            }
                        />
                    ))}
                </tbody>
            </table>
        </div>
    );
}
