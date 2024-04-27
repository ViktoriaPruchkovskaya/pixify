export default function EmbroideryCell({color, symbol}: { color: number[], symbol: string }) {
    return <td style={{
        backgroundColor: `rgba(${color.toString()},0.5)`, width: '20px',
        height: '20px',
        border: "solid black"
    }}>{symbol}</td>
}