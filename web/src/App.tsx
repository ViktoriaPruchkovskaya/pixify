import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";
import EmbroideryCanvas from "./components/EmbroideryCanvas/EmbroideryCanvas";
import {Canvas} from "./services/imageService";

export default function App() {
    const [canvas, setCanvas] = useState<Canvas>({
        embroidery: [],
        palette: [{color: {name: "", rgb: [], thread_length: 0}, identifier: "00"}]
    });
    const displayCanvas = () => {
        if (canvas?.embroidery.length) {
            return <EmbroideryCanvas canvas={canvas} onCanvasChange={setCanvas}/>
        }
    }

    return (
        <div>
            <ImageForm onCanvasReceived={setCanvas}/>
            {(displayCanvas)()}
        </div>
    );
}