import CanvasService from '../../services/canvasService';

interface ClearHistoryButtonProps {
    setCanvasNames: (canvasNames: string[]) => void;
}

export default function ClearHistoryButton({
    setCanvasNames,
}: ClearHistoryButtonProps) {
    const handleOnClick = () => {
        (async function () {
            const canvasService = new CanvasService();
            await canvasService.deleteCanvases();
            setCanvasNames([]);
        })();
    };

    return (
        <button
            onClick={handleOnClick}
            style={{
                width: '10%',
            }}
        >
            Clear history
        </button>
    );
}
