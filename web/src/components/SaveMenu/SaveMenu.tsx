import {FormEvent} from "react";
import {Canvas} from "../../services/imageService";
import {StorageService} from "../../services/storageService";
import MenuButton from "./MenuButton";

interface SaveMenuProps {
    canvas: Canvas;
    setIsSaveMenuShowed: (isShowed: boolean) => void;
}

export default function SaveMenu({canvas, setIsSaveMenuShowed}: SaveMenuProps) {
    const closeMenu = () => {
        setIsSaveMenuShowed(false)
    }

    const handleForm = (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const {canvasName} = event.target as HTMLFormElement;
        (async () => {
            const storageService = await StorageService.getInstance();
            await storageService.setCanvas(canvas, canvasName.value);
        })()
        closeMenu()
    }

    return (
        <div style={{
            display: "block",
            position: "fixed",
            width: "400px",
            height: "180px",
            padding: "20px",
            paddingTop: "40px",
            backgroundColor: "#f2f2f2",
            borderRadius: "2px",
            boxShadow: "0 2px 9px rgba(0, 0, 0, 0.6)",
        }}>
            <form onSubmit={handleForm}>
                <label htmlFor="image">Canvas Name</label>
                <input type="text" id="canvasName" name="canvasName" defaultValue="canvas" style={{
                    width: "100%",
                    padding: "12px 20px",
                    margin: "8px 0",
                    border: "1px solid #ccc",
                    borderRadius: "4px",
                    boxSizing: "border-box"
                }}/>
                <span>hey</span>
                <div style={{display: "flex", justifyContent: "space-between"}}>
                    <MenuButton type={"button"} backgroundColor={"#a4a4a4"}
                                onClick={closeMenu}>Cancel</MenuButton>
                    <MenuButton type={"submit"} backgroundColor={"#4Caf50"}>Save</MenuButton>
                </div>
            </form>
        </div>
    )
}