import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";

export default function EmbroideryCanvas({canvas}: { canvas: Canvas }) {
    const palette: Map<string, string> = new Map();
    canvas.palette.forEach(({identifier, color}) => palette.set(color.rgb.toString(), identifier));

    return (
        <div style={{marginTop: "5px", overflow: "auto", width: "800px", height: "700px", border: "1px solid black"}}>
            <table
                style={{margin: "5px", borderCollapse: "collapse"}}>{canvas.embroidery.map((row, i) =>
                <EmbroideryRow key={i} row={row} palette={palette}/>)}</table>
        </div>);
}