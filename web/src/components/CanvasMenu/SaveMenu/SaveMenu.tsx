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
    const [formError, setFormError] = useState<{
        kind: 'conflict' | 'other';
        message: string;
    }>();

    const saveCanvas = async (canvas: Canvas, name: string) => {
        const canvasService = new CanvasService();
        await canvasService.addCanvas(canvas, name);
        setCanvasNames([...canvasNames, name]);
    };

    const overwriteCanvas = async (canvas: Canvas, name: string) => {
        const canvasService = new CanvasService();
        await canvasService.putCanvas(canvas, name);
    };

    const [onSave, setOnSave] = useState<
        (canvas: Canvas, name: string) => Promise<void>
    >(() => saveCanvas);

    const closeMenu = () => {
        hideOverlay();
        setIsSaveMenuShowed(false);
    };

    const handleForm = (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const { canvasName } = event.target as HTMLFormElement;
        (async () => {
            try {
                await onSave(canvas, canvasName.value);
                closeMenu();
            } catch (error: unknown) {
                if (error instanceof Error) {
                    if (error.message.includes('already exists')) {
                        setFormError({
                            kind: 'conflict',
                            message: error.message,
                        });
                        setOnSave(() => overwriteCanvas);
                    } else {
                        setFormError({
                            kind: 'other',
                            message: error.message,
                        });
                    }
                }
            }
        })();
    };

    const resetState = () => {
        setFormError(undefined);
        setOnSave(() => saveCanvas);
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
                    onInput={() => formError && resetState()}
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
                    <span style={{ color: 'red' }}>{formError?.message}</span>
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
                        {formError?.kind === 'conflict' ? 'Overwrite' : 'Save'}
                    </MenuButton>
                </div>
            </form>
        </div>
    );
}
