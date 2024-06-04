import EmbroideryCell from './EmbroideryCell';

interface EmbroideryRowProps {
    row: number[][];
    palette: Map<string, string>;
    updateCanvas: (cellId: number) => (color: number[]) => void;
    changeCanvasUpdater: (cb: (color: number[]) => void) => void;
    showMenu: (xPos: number, yPos: number) => void;
    setSelectedCellPosition: (column: number) => () => void;
    selectedCellPosition?: { row: number; column: number };
}

export default function EmbroideryRow({
    row,
    palette,
    updateCanvas,
    changeCanvasUpdater,
    showMenu,
    setSelectedCellPosition,
    selectedCellPosition,
}: EmbroideryRowProps) {
    const getIdentifier = (color: number[]): string => {
        const colorString = color.toString();
        return palette.has(colorString) ? palette.get(colorString)! : '';
    };

    return (
        <tr>
            {row.map((color, i) => (
                <EmbroideryCell
                    key={i}
                    color={color}
                    identifier={getIdentifier(color)}
                    updateCanvas={updateCanvas(i)}
                    changeCanvasUpdater={changeCanvasUpdater}
                    showMenu={showMenu}
                    setSelectedCellPosition={setSelectedCellPosition(i)}
                    isSelected={selectedCellPosition?.column === i ?? false}
                />
            ))}
        </tr>
    );
}
