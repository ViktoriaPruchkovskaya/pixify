import { CSSProperties } from 'react';
import './ColorSelector.css';
import { PaletteColor } from '../../services/imageService';

interface ColorSelectorProps {
    updateCanvas: (color: number[]) => void;
    palette: PaletteColor[];
    dynamicStyles: CSSProperties;
}

export default function ColorSelector({
    dynamicStyles,
    updateCanvas,
    palette,
}: ColorSelectorProps) {
    const displaySelector = () => {
        return palette.map((color, i) => (
            <div
                key={i}
                className='color-container'
                onClick={() => updateCanvas(color.color.rgb)}
            >
                <div
                    className='color-box'
                    style={{
                        backgroundColor: `rgba(${color.color.rgb.toString()}, 0.5)`,
                    }}
                >
                    <span style={{ fontSize: 10, textAlign: 'center' }}>
                        {color.identifier}
                    </span>
                </div>
                <span style={{ fontSize: 12 }}>{color.color.name}</span>
            </div>
        ));
    };

    return (
        <div
            role={'dialog'}
            className='color-selector-container'
            style={dynamicStyles}
        >
            {displaySelector()}
        </div>
    );
}
