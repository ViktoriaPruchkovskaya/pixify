import {PaletteColor} from "../../services/imageService";
import {CSSProperties} from "react";


interface ColorSelectorProps {
    updateCanvas: (color: number[]) => void;
    palette: PaletteColor[];
    dynamicStyles: CSSProperties;
}

export default function ColorSelector({dynamicStyles, updateCanvas, palette}: ColorSelectorProps) {
    const displaySelector = () => {
        return palette.map((color, i) => (
            <div key={i} style={{margin: "4px", display: "flex", alignItems: "center", cursor: "pointer"}}
                 onClick={() => updateCanvas(color.color.rgb)
                 }>
                <div style={{
                    backgroundColor: `rgba(${color.color.rgb.toString()}, 0.5)`,
                    width: "15px",
                    height: "15px",
                    marginRight: "4px",
                    border: "dotted grey"
                }}>
                    <span style={{fontSize: 10, textAlign: "center"}}>{color.identifier}</span>
                </div>
                <span style={{fontSize: 12}}>{color.color.name}</span>
            </div>))
    }

    return (
        <div role={"dialog"} style={{
            position: "absolute",
            backgroundColor: "#f9f9f9",
            width: "100px",
            height: "100px",
            padding: "5px",
            borderRadius: "1px",
            boxShadow: "0px 8px 16px 0px rgba(0,0,0,0.2)",
            overflow: "auto",
            ...dynamicStyles
        }}>
            {displaySelector()}
        </div>)
}