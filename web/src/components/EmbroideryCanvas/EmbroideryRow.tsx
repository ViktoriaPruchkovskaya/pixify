import EmbroideryCell from "./EmbroideryCell";

export default function EmbroideryRow({row}: { row: number[][] }) {
    return (<tr>{row.map((cell, index) => <EmbroideryCell color={cell} order={index}/>)}</tr>)
}