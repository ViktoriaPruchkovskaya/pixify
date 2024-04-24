import EmbroideryCell from "./EmbroideryCell";

export default function EmbroideryRow({row, palette}: { row: number[][], palette: Map<number[], number> }) {
    const getSymbol = (color: number[]): string =>
        palette.has(color) ? palette.get(color).toString() : ""
    return (<tr>{row.map((cell) => <EmbroideryCell color={cell} symbol={getSymbol(cell)}/>)}</tr>)
}