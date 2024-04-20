import EmbroideryRow from "./EmbroideryRow";

export default function EmbroideryCanvas({canvas}: { canvas: number[][][] }) {
    return (<table>{canvas.map(row => <EmbroideryRow row={row}/>)}</table>);
}