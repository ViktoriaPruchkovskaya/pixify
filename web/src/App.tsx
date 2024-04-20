import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";
import EmbroideryCanvas from "./components/EmbroideryCanvas/EmbroideryCanvas";

export default function App() {
    const [image, setImage] = useState<number[][][]>([]);
    const displayCanvas = () => {
        if (image.length) {
            return <EmbroideryCanvas canvas={image}/>
        }
    }
    return (
        <div>
            <ImageForm onImageReceived={setImage}/>
            {(displayCanvas)()}
        </div>
    );
}