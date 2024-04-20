export default function EmbroideryCell({color, order}: { color: number[], order: number }) {
    return <td style={{
        backgroundColor: `rgba(${color[0]}, ${color[1]},${color[2]},0.5)`, width: '10px',
        height: '10px',
        border: "1px solid black"
    }}>{order}</td>
}