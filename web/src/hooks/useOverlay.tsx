import { useState } from 'react';

interface Overlay {
    showOverlay: (onClose: () => void) => void;
    hideOverlay: () => void;
    isOverlayRendered: boolean;
    onClose?: () => void;
}

export function useOverlay(): Overlay {
    const [isOverlayRendered, setIsOverlayRendered] = useState(false);
    const [onClose, setOnClose] = useState<() => void>();
    const showOverlay = (onClose: () => void) => {
        setIsOverlayRendered(true);
        setOnClose(() => onClose);
    };
    const hideOverlay = () => {
        setIsOverlayRendered(false);
        setOnClose(undefined);
    };

    return { showOverlay, hideOverlay, isOverlayRendered, onClose };
}
