import EmbroideryCell from "./EmbroideryCell";

export default function EmbroideryRow({row, palette}: { row: number[][], palette: Map<string, number> }) {
    const getSymbol = (color: number[]): string => {
        const colorString = color.toString();
        return palette.has(colorString) ? palette.get(colorString)!.toString() : "";
    }

    return (<tr>{row.map((cell, i) => <EmbroideryCell key={i} color={cell} symbol={getSymbol(cell)}/>)}</tr>)
}