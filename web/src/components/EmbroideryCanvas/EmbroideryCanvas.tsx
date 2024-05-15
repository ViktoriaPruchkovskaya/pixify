import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";

export default function EmbroideryCanvas({canvas}: { canvas: Canvas }) {
    const palette: Map<string, string> = new Map();
    canvas.palette.forEach(({identifier, color}) => palette.set(color.rgb.toString(), identifier));

    return (
        <table>{canvas.embroidery.map((row, i) => <EmbroideryRow key={i} row={row} palette={palette}/>)}</table>);
}