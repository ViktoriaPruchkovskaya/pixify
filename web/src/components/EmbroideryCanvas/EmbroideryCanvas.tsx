import EmbroideryRow from "./EmbroideryRow";
import {Canvas} from "../../services/imageService";

export default function EmbroideryCanvas({canvas}: { canvas: Canvas }) {
    let palette: Map<number[], number> = new Map(); //rgb -> symbol=0......1000
    canvas.palette.forEach(({symbol, color}) => palette.set(color.rgb, symbol))
    console.log("hello")
    return (<table>{canvas.embroidery.map(row => <EmbroideryRow row={row} palette={palette}/>)}</table>);
}