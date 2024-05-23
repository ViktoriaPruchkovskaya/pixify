import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";
import EmbroideryCanvas from "./components/EmbroideryCanvas/EmbroideryCanvas";
import {Canvas} from "./services/imageService";
import ColorSelector from "./components/ColorSelector/ColorSelector";
import {useColorContextMenu} from "./hooks/useColorContextMenu";

export default function App() {
    const [canvas, setCanvas] = useState<Canvas>({
        embroidery: [],
        palette: [{color: {name: "", rgb: [], thread_length: 0}, identifier: "00"}]
    });

    const {
        canvasUpdater,
        setCanvasUpdater,
        isMenuShowed,
        showMenu,
        hideMenu,
        selectorStyle
    } = useColorContextMenu();


    return (
        <div>
            {isMenuShowed && <div style={{
                display: "block",
                position: "fixed",
                width: "100vw",
                height: "100vh",
                top: 0, left: 0,
            }} onClick={() => {
                hideMenu();
            }}/>}
            <ColorSelector dynamicStyles={selectorStyle} palette={canvas.palette} updateCanvas={canvasUpdater}/>
            <ImageForm onCanvasReceived={setCanvas}/>
            {canvas?.embroidery.length ? <EmbroideryCanvas style={isMenuShowed ? {pointerEvents: 'none'} : {}}
                                                           canvas={canvas} onCanvasChange={setCanvas}
                                                           changeCanvasUpdater={setCanvasUpdater}
                                                           showMenu={showMenu}
            /> : undefined}
        </div>
    );
}