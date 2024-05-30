import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";
import EmbroideryCanvas from "./components/EmbroideryCanvas/EmbroideryCanvas";
import {Canvas} from "./services/imageService";
import ColorSelector from "./components/ColorSelector/ColorSelector";
import {useColorContextMenu} from "./hooks/useColorContextMenu";
import {useSelectedCell} from "./hooks/useSelectedCell";
import ThreadsPalette from "./components/ThreadsPalette/ThreadsPalette";
import SaveButton from "./components/SaveMenu/SaveButton";
import CanvasMenu from "./components/CanvasMenu/CanvasMenu";

export default function App() {
    const [canvas, setCanvas] = useState<Canvas>({
        embroidery: [],
        palette: [{identifier: "00", color: {name: "", rgb: []}, n_stitches: 0}]
    });

    const {
        canvasUpdater,
        setCanvasUpdater,
        isMenuShowed,
        showMenu,
        hideMenu,
        selectorStyle
    } = useColorContextMenu();

    const {setSelectedCellPosition, resetCellPosition, selectedCellPosition} = useSelectedCell();
    const [isSaveMenuShowed, setIsSaveMenuShowed] = useState(false);

    return (
        <div>
            {isMenuShowed && <div style={{
                display: "block",
                position: "fixed",
                width: "100vw",
                height: "100vh",
                top: 0,
                left: 0,
            }} onClick={() => {
                hideMenu();
                resetCellPosition();
            }}/>}
            <ColorSelector dynamicStyles={selectorStyle} palette={canvas.palette} updateCanvas={canvasUpdater}/>
            <ImageForm onCanvasReceived={setCanvas}/>
            <CanvasMenu onCanvasChange={setCanvas}/>
            {canvas?.embroidery.length ? (
                <div style={{
                    marginTop: "15px",
                    display: "flex",
                    justifyContent: "space-around",
                    paddingLeft: "20px",
                    paddingRight: "20px"
                }}>
                    <EmbroideryCanvas style={isMenuShowed ? {pointerEvents: 'none'} : {}}
                                      canvas={canvas} onCanvasChange={setCanvas}
                                      changeCanvasUpdater={setCanvasUpdater}
                                      showMenu={showMenu}
                                      setSelectedCellPosition={setSelectedCellPosition}
                                      selectedCellPosition={selectedCellPosition}/>
                    <ThreadsPalette palette={canvas.palette}/>
                    <SaveButton canvas={canvas} isSaveMenuShowed={isSaveMenuShowed}
                                setIsSaveMenuShowed={setIsSaveMenuShowed}/>
                </div>
            ) : undefined}
        </div>
    );
}