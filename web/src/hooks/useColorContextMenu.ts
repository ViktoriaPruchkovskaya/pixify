import { CSSProperties, Dispatch, SetStateAction, useState } from 'react';

interface ColorContextMenu {
    canvasUpdater: (color: number[]) => void;
    setCanvasUpdater: Dispatch<SetStateAction<(color: number[]) => void>>;
    colorSelectorStyle: CSSProperties;
    showMenu: (xPos: number, yPos: number) => void;
}

export function useColorContextMenu(
    showOverlay: (onOverlayHide: () => void) => void,
    resetCellPosition: () => void
): ColorContextMenu {
    const [canvasUpdater, setCanvasUpdater] = useState<
        (color: number[]) => void
    >(() => () => {});

    const [colorSelectorStyle, setColorSelectorStyle] = useState<CSSProperties>(
        { display: 'none' }
    );

    const hideMenu = () => {
        setColorSelectorStyle({ display: 'none' });
    };

    const showMenu = (xPos: number, yPos: number) => {
        setColorSelectorStyle({
            display: 'block',
            top: yPos,
            left: xPos,
            zIndex: 1,
        });
        showOverlay(() => {
            hideMenu();
            resetCellPosition();
        });
    };

    return { canvasUpdater, setCanvasUpdater, colorSelectorStyle, showMenu };
}
