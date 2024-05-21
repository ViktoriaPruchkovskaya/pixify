import {FormEvent, useContext} from "react";
import {PaletteContext} from "../../contexts/paletteContext";


interface ColorSelectorProps {
    updateCanvas: (color: number[]) => void;
}

export default function ColorSelector({updateCanvas}: ColorSelectorProps) {
    const palette = useContext(PaletteContext);

    let handleOnClick = (event: FormEvent<HTMLDivElement>) => {
        const target = event.currentTarget.querySelector('div');
        if (target) {
            const styles = window.getComputedStyle(target as HTMLDivElement);
            const colors = styles.backgroundColor.match(/\d+(\.\d+)?/gi);
            if (!colors || colors.length < 3) {
                throw new Error("Failed to change cell color");
            }
            const [r, g, b] = colors.map(el => parseInt(el));
            updateCanvas([r, g, b])
        }
    }

    const displaySelector = () => (
        palette.map(color => (
            <div style={{margin: "4px", display: "flex", alignItems: "center"}} onClick={handleOnClick}>
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
            </div>)))

    return (
        <div style={{
            position: "absolute",
            backgroundColor: "#f9f9f9",
            width: "100px",
            height: "100px",
            padding: "5px",
            borderRadius: "1px",
            boxShadow: "0px 8px 16px 0px rgba(0,0,0,0.2)",
            overflow: "auto"
        }}>{displaySelector()}
        </div>)
}