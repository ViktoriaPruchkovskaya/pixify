import {PaletteColor} from "../../services/imageService";

interface ThreadPaletteProps {
    palette: PaletteColor[];
}

export default function ThreadsPalette({palette}: ThreadPaletteProps) {
    const calculateThreadLength = (stitches: number) => stitches + stitches * 0.2;

    return (<div
        style={{
            display: "grid",
            height: "700px",
            width: "350px",
            overflow: "auto",
            gap: "10px",
            boxShadow: "0 2px 9px rgba(0, 0, 0, 0.3)",
            padding: "10px",
            boxSizing: "border-box"
        }}>
        {palette.map(thread => (
            <div style={{
                display: "grid",
                gap: "5%",
                gridTemplateColumns: '120px auto 50px',
                placeItems: "center"
            }}>
                <p style={{textAlign: "center"}}>Color {thread.identifier}: <a
                    target="_blank"
                    href={`https://stitchpalettes.com/embroidery-thread-color-schemes/?threadcode=${thread.color.name}`}>#{thread.color.name}</a>
                </p>
                <div style={{
                    display: "flex",
                    justifyContent: "center",
                    alignItems: "center"
                }}>
                    <div style={{
                        width: "90px",
                        height: "30px",
                        backgroundColor: `rgba(${thread.color.rgb.toString()},1)`,
                        boxShadow: "0 2px 2px rgba(0, 0, 0, 0.1)",
                    }}/>
                </div>
                <p>{calculateThreadLength(thread.n_stitches)}cm</p>
            </div>))}
    </div>)
}
