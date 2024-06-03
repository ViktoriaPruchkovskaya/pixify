import {Canvas} from "../../../services/imageService";
import SaveMenu from "./SaveMenu";
import {useState} from "react";

interface SaveButtonProps {
    canvas: Canvas;
    showOverlay: (onClose: () => void) => void;
    hideOverlay: () => void;
}

export default function SaveButton(
    {
        canvas,
        showOverlay,
        hideOverlay
    }: SaveButtonProps) {
    const [isSaveMenuShowed, setIsSaveMenuShowed] = useState(false);
    const handleOnClick = () => {
        setIsSaveMenuShowed(true);
        showOverlay(() => {
            setIsSaveMenuShowed(false);
        });
    }

    return (<>
        <button onClick={handleOnClick}>SAVE</button>
        {isSaveMenuShowed &&
            <SaveMenu canvas={canvas} setIsSaveMenuShowed={setIsSaveMenuShowed} hideOverlay={hideOverlay}/>}
    </>)
}