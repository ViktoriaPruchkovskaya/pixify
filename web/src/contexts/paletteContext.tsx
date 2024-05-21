import {createContext} from "react";

interface PaletteContext {
    identifier: string;
    color: { rgb: number[]; name: string; thread_length: number; };
}

export const PaletteContext = createContext<PaletteContext[]>([]);