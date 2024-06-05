import { FormEvent, useState } from 'react';
import MenuButton from './MenuButton';
import { Canvas } from '../../../services/imageService';
import CanvasService from '../../../services/canvasService';

interface SaveMenuProps {
    canvas: Canvas;
    setIsSaveMenuShowed: (isShowed: boolean) => void;
    hideOverlay: () => void;
    setCanvasNames: (canvasNames: string[]) => void;
    canvasNames: string[];
}

export default function SaveMenu({
    canvas,
    setIsSaveMenuShowed,
    hideOverlay,
    canvasNames,
    setCanvasNames,
}: SaveMenuProps) {
    const [formError, setFormError] = useState<string>();
    const closeMenu = () => {
        hideOverlay();
        setIsSaveMenuShowed(false);
    };

    const handleForm = (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const { canvasName } = event.target as HTMLFormElement;
        (async () => {
            const canvasService = new CanvasService();
            try {
                await canvasService.addCanvas(canvas, canvasName.value);
                setCanvasNames([...canvasNames, canvasName.value]);
                closeMenu();
            } catch (error: unknown) {
                if (error instanceof Error) {
                    setFormError(error.message);
                }
            }
        })();
    };

    return (
        <div
            style={{
                top: '10%',
                left: '35%',
                display: 'block',
                position: 'fixed',
                margin: '0 0 auto',
                width: '400px',
                height: '180px',
                padding: '20px',
                paddingTop: '40px',
                backgroundColor: '#f2f2f2',
                zIndex: '1',
                borderRadius: '2px',
                boxShadow: '0 2px 9px rgba(0, 0, 0, 0.6)',
            }}
        >
            <form onSubmit={handleForm}>
                <label htmlFor='image'>Canvas Name</label>
                <input
                    type='text'
                    id='canvasName'
                    name='canvasName'
                    defaultValue='canvas'
                    onInput={() => formError && setFormError(undefined)}
                    style={{
                        width: '100%',
                        padding: '12px 20px',
                        margin: '8px 0',
                        border: '1px solid #ccc',
                        borderRadius: '4px',
                        boxSizing: 'border-box',
                    }}
                />
                <div style={{ width: '100%', height: '20px' }}>
                    <span style={{ color: 'red' }}>{formError}</span>
                </div>
                <div
                    style={{ display: 'flex', justifyContent: 'space-between' }}
                >
                    <MenuButton
                        type={'button'}
                        backgroundColor={'#a4a4a4'}
                        onClick={closeMenu}
                    >
                        Cancel
                    </MenuButton>
                    <MenuButton type={'submit'} backgroundColor={'#4Caf50'}>
                        Save
                    </MenuButton>
                </div>
            </form>
        </div>
    );
}
