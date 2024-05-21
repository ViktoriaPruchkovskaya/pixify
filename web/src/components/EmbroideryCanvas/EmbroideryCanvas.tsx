import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";
import {PaletteContext} from "../../contexts/paletteContext";

interface EmbroideryCanvasProps {
    canvas: Canvas,
    onCanvasChange: (canvas: Canvas) => void;
}

export default function EmbroideryCanvas({canvas, onCanvasChange}: EmbroideryCanvasProps) {
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
        <PaletteContext.Provider value={canvas.palette}>
            <div style={{
                marginTop: "5px",
                overflow: "auto",
                width: "800px",
                height: "700px",
                border: "1px solid black",
                position: "relative"
            }}>
                <table
                    style={{
                        margin: "5px",
                        borderCollapse: "collapse",
                        position: "relative"
                    }}>
                    <tbody>
                    {canvas.embroidery.map((row, i) =>
                        <EmbroideryRow key={i} row={row} palette={getPalette()} updateCanvas={updateCanvas(i)}/>)}
                    </tbody>
                </table>
            </div>
        </PaletteContext.Provider>);
}