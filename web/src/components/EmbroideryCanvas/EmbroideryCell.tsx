import './EmbroideryCell.css';

interface EmbroideryCell {
    color: number[];
    identifier: string;
    updateCanvas: (color: number[]) => void;
    changeCanvasUpdater: (cb: (color: number[]) => void) => void;
    showMenu: (xPos: number, yPos: number) => void;
    setSelectedCellPosition: () => void;
    isSelected?: boolean;
}

export default function EmbroideryCell({
    color,
    identifier,
    updateCanvas,
    changeCanvasUpdater,
    showMenu,
    setSelectedCellPosition,
    isSelected,
}: EmbroideryCell) {
    const handleOnClick = (event: React.MouseEvent<HTMLTableCellElement>) => {
        const targetPosition = event.currentTarget.getBoundingClientRect();
        showMenu(targetPosition.x + 35, targetPosition.y + 5);
        setSelectedCellPosition();
        changeCanvasUpdater(() => updateCanvas);
    };

    return (
        <td
            className='embroidery-cell'
            style={{
                backgroundColor: `rgba(${color.toString()},0.5)`,
                transform: isSelected ? 'scale(1.5,1.4)' : undefined,
            }}
            onClick={handleOnClick}
        >
            {identifier}
        </td>
    );
}
