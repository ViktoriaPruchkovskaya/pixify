import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";
import EmbroideryCanvas from "./components/EmbroideryCanvas/EmbroideryCanvas";
import {Canvas} from "./services/imageService";
import ColorSelector from "./components/ColorSelector/ColorSelector";
import {useColorContextMenu} from "./hooks/useColorContextMenu";
import {useSelectedCell} from "./hooks/useSelectedCell";
import ThreadsPalette from "./components/ThreadsPalette/ThreadsPalette";
import CanvasMenu from "./components/CanvasMenu/CanvasMenu";
import {useOverlay} from "./hooks/useOverlay";

export default function App() {
    const [canvas, setCanvas] = useState<Canvas>({
        embroidery: [],
        palette: [{identifier: "00", color: {name: "", rgb: []}, n_stitches: 0}]
    });

    const {showOverlay, hideOverlay, isOverlayRendered, onClose} = useOverlay();
    const {setSelectedCellPosition, resetCellPosition, selectedCellPosition} = useSelectedCell();
    const {
        canvasUpdater,
        setCanvasUpdater,
        showMenu,
        colorSelectorStyle
    } = useColorContextMenu(showOverlay, resetCellPosition);

    return (
        <div>
            {isOverlayRendered && <div style={{
                display: "block",
                position: "fixed",
                width: "100vw",
                height: "100vh",
                top: 0,
                left: 0,
            }} onClick={() => {
                onClose && onClose();
                hideOverlay();
            }}/>}
            <ColorSelector dynamicStyles={colorSelectorStyle} palette={canvas.palette} updateCanvas={canvasUpdater}/>
            <div style={{display: "flex", flexDirection: "column"}}>
                <ImageForm onCanvasReceived={setCanvas}/>
                <CanvasMenu onCanvasSelected={setCanvas} canvas={canvas}
                            showOverlay={showOverlay}
                            hideOverlay={hideOverlay}/>
            </div>

            {canvas?.embroidery.length ? (
                <div style={{
                    marginTop: "15px",
                    display: "flex",
                    justifyContent: "space-around",
                    paddingLeft: "20px",
                    paddingRight: "20px"
                }}>
                    <EmbroideryCanvas style={isOverlayRendered ? {pointerEvents: 'none'} : {}}
                                      canvas={canvas} onCanvasChange={setCanvas}
                                      changeCanvasUpdater={setCanvasUpdater}
                                      showMenu={showMenu}
                                      setSelectedCellPosition={setSelectedCellPosition}
                                      selectedCellPosition={selectedCellPosition}/>
                    <ThreadsPalette palette={canvas.palette}/>
                </div>
            ) : undefined}
        </div>
    );
}