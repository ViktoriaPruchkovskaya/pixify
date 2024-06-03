import CanvasSelector from "./CanvasSelector";
import {Canvas} from "../../services/imageService";
import SaveButton from "./SaveMenu/SaveButton";

interface CanvasMenuProps {
    onCanvasSelected: (canvas: Canvas) => void;
    canvas: Canvas;
    setIsSaveMenuShowed: (isShowed: boolean) => void;
    isSaveMenuShowed: boolean;
}

export default function CanvasMenu({onCanvasSelected, canvas, setIsSaveMenuShowed, isSaveMenuShowed}: CanvasMenuProps) {
    return <div>
        <CanvasSelector onCanvasSelected={onCanvasSelected}/>
        <SaveButton canvas={canvas} setIsSaveMenuShowed={setIsSaveMenuShowed} isSaveMenuShowed={isSaveMenuShowed}/>
    </div>;
}