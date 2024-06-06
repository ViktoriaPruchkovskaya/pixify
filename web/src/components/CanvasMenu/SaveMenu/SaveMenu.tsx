import { FormEvent, useState } from 'react';
import './SaveMenu.css';
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
        <div className='save-menu-window'>
            <form onSubmit={handleForm}>
                <label htmlFor='image'>Canvas Name</label>
                <input
                    className='canvas-name-input'
                    type='text'
                    id='canvasName'
                    name='canvasName'
                    defaultValue='canvas'
                    onInput={() => formError && resetState()}
                />
                <div className='error-message-container'>
                    <span style={{ color: 'red' }}>{formError?.message}</span>
                </div>
                <div className='menu-buttons-container'>
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
