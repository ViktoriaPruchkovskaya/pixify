import {Canvas} from "../../services/imageService";
import SaveMenu from "./SaveMenu";

interface SaveButtonProps {
    canvas: Canvas;
    setIsSaveMenuShowed: (isShowed: boolean) => void;
    isSaveMenuShowed: boolean;
}

export default function SaveButton({canvas, setIsSaveMenuShowed, isSaveMenuShowed}: SaveButtonProps) {
    const handleOnClick = () => {
        setIsSaveMenuShowed(true);
    }

    return (<>
        <button onClick={handleOnClick}>SAVE</button>
        {isSaveMenuShowed && <SaveMenu canvas={canvas} setIsSaveMenuShowed={setIsSaveMenuShowed}/>}
    </>)
}