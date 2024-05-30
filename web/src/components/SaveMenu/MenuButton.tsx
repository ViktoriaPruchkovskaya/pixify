interface FormButtonInterface {
    type: "submit" | "button";
    backgroundColor: string;
    onClick?: () => void;
    children: string
}

export default function MenuButton({type, children, backgroundColor, onClick}: FormButtonInterface) {
    return (<button type={type} style={{
        width: "48%",
        padding: "10px",
        border: "none",
        borderRadius: "4px",
        backgroundColor,
        color: "white",
        cursor: "pointer",
    }} onClick={onClick}>{children}</button>)
}