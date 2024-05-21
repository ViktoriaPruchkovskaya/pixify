import {useState} from "react";
import ColorSelector from "./ColorSelector";

interface EmbroideryCell {
    color: number[],
    identifier: string,
    updateCanvas: (color: number[]) => void
}

export default function EmbroideryCell({color, identifier, updateCanvas}: EmbroideryCell) {
    let [isFocused, setIsFocused] = useState(false);
    let [isSelected, setIsSelected] = useState(false);

    const handleOnFocus = () => {
        setIsFocused(true)
    }

    const handleOnLeave = () => {
        setIsSelected(false)
        setIsFocused(false)
    }

    const handleOnClick = () => {
        setIsSelected(true)
    }

    return <td style={{
        backgroundColor: `rgba(${color.toString()},0.5)`,
        width: '25px',
        height: '25px',
        border: "solid black",
        textAlign: "center",
        minWidth: "25px",
        minHeight: "25px",
        transform: isFocused ? "scale(1.5,1.4)" : undefined,
        cursor: "pointer"
    }} onMouseOver={handleOnFocus} onMouseLeave={handleOnLeave} onClick={handleOnClick}>{identifier} {isSelected &&
        <ColorSelector updateCanvas={updateCanvas}/>}
    </td>
}