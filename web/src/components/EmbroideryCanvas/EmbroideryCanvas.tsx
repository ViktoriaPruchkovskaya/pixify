import {CSSProperties} from "react";
import EmbroideryRow from "./EmbroideryRow";
import {Canvas, PaletteColor} from "../../services/imageService";

interface EmbroideryCanvasProps {
    canvas: Canvas;
    onCanvasChange: (canvas: Canvas) => void;
    changeCanvasUpdater: (cb: (color: number[]) => void) => void;
    showMenu: (xPos: number, yPos: number) => void;
    style?: CSSProperties;
    setSelectedCellPosition: (row: number) => (column: number) => () => void;
    selectedCellPosition?: { row: number, column: number }
}

export default function EmbroideryCanvas(
    {
        canvas,
        onCanvasChange,
        changeCanvasUpdater,
        showMenu,
        style,
        setSelectedCellPosition,
        selectedCellPosition
    }: EmbroideryCanvasProps) {
    const getPalette = () => {
        const palette: Map<string, string> = new Map();
        canvas.palette.forEach(({identifier, color}) => palette.set(color.rgb.toString(), identifier));
        return palette;
    }

    const updateCanvas = (rowId: number) => {
        const newEmbroidery: number[][][] = [...canvas.embroidery];
        const newPalette: PaletteColor[] = [...canvas.palette];
        const isEqual = (arr1: number[], arr2: number[]): boolean => arr1.every((el, index) => el === arr2[index]);

        return (cellId: number) => {
            return (rgb: number[]) => {
                const oldColor: number[] = newEmbroidery[rowId][cellId];
                newEmbroidery[rowId][cellId] = rgb;
                for (const thread of newPalette) {
                    if (isEqual(thread.color.rgb, rgb)) {
                        thread.thread_length += 1;
                    }
                    if (isEqual(thread.color.rgb, oldColor)) {
                        thread.thread_length -= 1;
                    }
                }
                canvas.embroidery = newEmbroidery;
                onCanvasChange({...canvas})
            }
        }
    }

    return (
        <div style={{
            overflow: "auto",
            height: "700px",
            boxShadow: "0 2px 9px rgba(0, 0, 0, 0.3)",
            position: "relative",
            ...style
        }}>
            <table
                style={{
                    margin: "5px",
                    borderCollapse: "collapse",
                    position: "relative"
                }}>
                <tbody>
                {canvas.embroidery.map((row, i) =>
                    <EmbroideryRow key={i} row={row}
                                   palette={getPalette()} updateCanvas={updateCanvas(i)}
                                   changeCanvasUpdater={changeCanvasUpdater}
                                   showMenu={showMenu} setSelectedCellPosition={setSelectedCellPosition(i)}
                                   selectedCellPosition={selectedCellPosition?.row === i ? selectedCellPosition : undefined}/>)}
                </tbody>
            </table>
        </div>);
}