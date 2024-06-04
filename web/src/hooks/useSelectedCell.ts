import { useState } from 'react';

interface CellPosition {
    row: number;
    column: number;
}

interface SelectedCell {
    selectedCellPosition?: CellPosition;
    setSelectedCellPosition: (row: number) => (column: number) => () => void;
    resetCellPosition: () => void;
}

export function useSelectedCell(): SelectedCell {
    const [selectedCellPosition, setCellPosition] = useState<CellPosition>();

    const setSelectedCellPosition = (row: number) => {
        return (column: number) => {
            return () => {
                setCellPosition({ row, column });
            };
        };
    };

    const resetCellPosition = () => {
        setCellPosition(undefined);
    };

    return { selectedCellPosition, setSelectedCellPosition, resetCellPosition };
}
