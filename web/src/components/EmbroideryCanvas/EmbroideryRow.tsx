import EmbroideryCell from "./EmbroideryCell";

interface EmbroideryRowProps {
    row: number[][],
    palette: Map<string, string>,
    updateCanvas: (cellId: number) => (color: number[]) => void
}

export default function EmbroideryRow({row, palette, updateCanvas}: EmbroideryRowProps) {
    const getIdentifier = (color: number[]): string => {
        const colorString = color.toString();
        return palette.has(colorString) ? palette.get(colorString)! : "";
    }

    return (<tr>{row.map((color, i) =>
        <EmbroideryCell key={i} updateCanvas={updateCanvas(i)} color={color} identifier={getIdentifier(color)}/>)}
    </tr>)
}