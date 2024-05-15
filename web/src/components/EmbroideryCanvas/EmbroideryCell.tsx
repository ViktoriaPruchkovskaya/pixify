export default function EmbroideryCell({color, identifier}: { color: number[], identifier: string }) {
    return <td style={{
        backgroundColor: `rgba(${color.toString()},0.5)`, width: '25px',
        height: '25px',
        border: "solid black",
        textAlign: "center",
        minWidth: "25px",
        minHeight: "25px"
    }}>{identifier}</td>
}