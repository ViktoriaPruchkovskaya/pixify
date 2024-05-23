import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";
import {CSSProperties} from "react";

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
        let newEmbroidery: number[][][] = [...canvas.embroidery];
        return (cellId: number) => {
            return (rgb: number[]) => {
                newEmbroidery[rowId][cellId] = rgb;
                canvas.embroidery = newEmbroidery;
                onCanvasChange({...canvas})
            }
        }
    }

    return (
        <div style={{
            marginTop: "5px",
            overflow: "auto",
            width: "800px",
            height: "700px",
            border: "1px solid black",
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