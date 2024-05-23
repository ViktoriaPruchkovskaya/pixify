import {useState} from "react";

interface EmbroideryCell {
    color: number[];
    identifier: string;
    updateCanvas: (color: number[]) => void;
    changeCanvasUpdater: (cb: (color: number[]) => void) => void;
    showMenu: (xPos: number, yPos: number) => void;
    setSelectedCellPosition: () => void;
    isSelected?: boolean;
}

export default function EmbroideryCell(
    {
        color,
        identifier,
        updateCanvas,
        changeCanvasUpdater,
        showMenu,
        setSelectedCellPosition,
        isSelected
    }: EmbroideryCell) {
    const [isFocused, setIsFocused] = useState(false);
    const handleOnFocus = () => {
        setIsFocused(true)
    }

    const handleOnLeave = () => {
        setIsFocused(false)
    }

    const handleOnClick = (event: React.MouseEvent<HTMLTableCellElement>) => {
        const targetPosition = event.currentTarget.getBoundingClientRect();
        showMenu(targetPosition.x + 35, targetPosition.y + 5);
        setSelectedCellPosition();
        changeCanvasUpdater(() => updateCanvas)
    }

    return <td style={{
        backgroundColor: `rgba(${color.toString()},0.5)`,
        width: '25px',
        height: '25px',
        border: "solid black",
        textAlign: "center",
        minWidth: "25px",
        minHeight: "25px",
        transform: isFocused || isSelected ? "scale(1.5,1.4)" : undefined,
        cursor: "pointer"
    }} onMouseOver={handleOnFocus} onMouseLeave={handleOnLeave} onClick={handleOnClick}>{identifier}
    </td>
}