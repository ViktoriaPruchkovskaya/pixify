import {useState} from "react";
import ImageForm from "./components/ImageUpload/ImageForm";

export default function App() {
    const [image, setImage] = useState<Blob>();
    return (
        <div>
            <ImageForm onImageReceived={setImage}/>
            {image && <img src={URL.createObjectURL(image)}/>}
        </div>
    );
}