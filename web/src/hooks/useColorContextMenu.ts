import {CSSProperties, Dispatch, SetStateAction, useState} from "react";

interface ColorContextMenu {
    canvasUpdater: (color: number[]) => void;
    setCanvasUpdater: Dispatch<SetStateAction<(color: number[]) => void>>;
    selectorStyle: CSSProperties;
    isMenuShowed: boolean;
    hideMenu: () => void;
    showMenu: (xPos: number, yPos: number) => void;
}

export function useColorContextMenu(): ColorContextMenu {
    const [canvasUpdater, setCanvasUpdater] =
        useState<(color: number[]) => void>(() => () => {
        });

    const [selectorStyle, setSelectorStyle] = useState<CSSProperties>({display: "none"});
    const [isMenuShowed, setIsMenuShowed] = useState(false);

    const hideMenu = () => {
        setSelectorStyle({display: "none"});
        setIsMenuShowed(false);
    };

    const showMenu = (xPos: number, yPos: number) => {
        setIsMenuShowed(true);
        setSelectorStyle({
            display: "block",
            "top": yPos,
            "left": xPos,
            zIndex: 1000
        });
    };

    return {canvasUpdater, setCanvasUpdater, selectorStyle, isMenuShowed, showMenu, hideMenu};
}