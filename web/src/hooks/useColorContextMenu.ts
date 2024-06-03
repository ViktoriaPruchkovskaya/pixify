import {CSSProperties, Dispatch, SetStateAction, useState} from "react";

interface ColorContextMenu {
    canvasUpdater: (color: number[]) => void;
    setCanvasUpdater: Dispatch<SetStateAction<(color: number[]) => void>>;
    selectorStyle: CSSProperties;
    showMenu: (xPos: number, yPos: number) => void;
}

export function useColorContextMenu(showOverlay: (onOverlayHide: () => void) => void, resetCellPosition: () => void): ColorContextMenu {
    const [canvasUpdater, setCanvasUpdater] =
        useState<(color: number[]) => void>(() => () => {
        });

    const [selectorStyle, setSelectorStyle] = useState<CSSProperties>({display: "none"});

    const hideMenu = () => {
        setSelectorStyle({display: "none"});
    };

    const showMenu = (xPos: number, yPos: number) => {
        setSelectorStyle({
            display: "block",
            "top": yPos,
            "left": xPos,
            zIndex: 1000
        });
        showOverlay(() => {
            hideMenu();
            resetCellPosition();
        });
    };

    return {canvasUpdater, setCanvasUpdater, selectorStyle, showMenu};
}