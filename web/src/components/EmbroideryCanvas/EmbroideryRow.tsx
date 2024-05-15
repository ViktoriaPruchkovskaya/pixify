import EmbroideryCell from "./EmbroideryCell";

export default function EmbroideryRow({row, palette}: { row: number[][], palette: Map<string, string> }) {
    const getIdentifier = (color: number[]): string => {
        const colorString = color.toString();
        return palette.has(colorString) ? palette.get(colorString)! : "";
    }

    return (<tr>{row.map((cell, i) => <EmbroideryCell key={i} color={cell} identifier={getIdentifier(cell)}/>)}</tr>)
}