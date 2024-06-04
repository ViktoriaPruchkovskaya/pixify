import { Canvas } from './imageService';
import { StorageService } from './storageService';

interface CanvasEntry extends Canvas {
    id: string;
}

export default class CanvasService {
    private readonly storeName = 'canvases';

    public async addCanvas(canvas: Canvas, name: string): Promise<void> {
        const storeService = await StorageService.getInstance();
        const transaction = storeService.db.transaction(
            this.storeName,
            'readwrite'
        );
        const existingCanvas = await storeService.get({
            storeName: this.storeName,
            key: name,
            transaction,
        });
        if (existingCanvas) {
            throw new Error('Canvas with such name already exists');
        }
        await storeService.add<Canvas>({
            storeName: this.storeName,
            data: canvas,
            key: name,
            transaction,
        });
    }

    public async getCanvasNames(): Promise<string[]> {
        const storeService = await StorageService.getInstance();
        return storeService.getAllKeys<string[]>({ storeName: this.storeName });
    }

    public async getCanvasByName(name: string): Promise<CanvasEntry> {
        const storeService = await StorageService.getInstance();
        return storeService.get<CanvasEntry>({
            storeName: this.storeName,
            key: name,
        });
    }
}
