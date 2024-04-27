import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";

export default function EmbroideryCanvas({canvas}: { canvas: Canvas }) {
    const palette: Map<string, number> = new Map();
    canvas.palette.forEach(({symbol, color}) => palette.set(color.rgb.toString(), symbol));

    return (
        <table>{canvas.embroidery.map((row, i) => <EmbroideryRow key={i} row={row} palette={palette}/>)}</table>);
}