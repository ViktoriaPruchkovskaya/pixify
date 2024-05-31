import {Canvas} from "./imageService";

export class StorageService {
    private readonly db: IDBDatabase;
    public static instance: StorageService;

    private constructor(db: IDBDatabase) {
        this.db = db
    }

    static async getInstance() {
        if (StorageService.instance) {
            return StorageService.instance;
        }
        const db = await this.open();
        return StorageService.instance = new StorageService(db);
    }

    private static async open(): Promise<IDBDatabase> {
        return new Promise((resolve, reject) => {
            const request = window.indexedDB.open("store", 1);

            request.onupgradeneeded = function () {
                let db = request.result;
                if (!db.objectStoreNames.contains('canvases')) {
                    db.createObjectStore('canvases');
                }
            };

            request.onerror = () => {
                reject("Cannot open storage");
            };

            request.onsuccess = (event: Event) => {
                resolve((event.target as IDBOpenDBRequest).result);
            }

        })
    }

    public async setCanvas(canvas: Canvas, name: string): Promise<void> {
        const transaction = this.db.transaction("canvases", "readwrite");
        const existingCanvas = await this.get({collectionName: "canvases", key: name, transaction});
        if (existingCanvas) {
            throw new Error("Canvas with such name already exists");
        }
        await this.add({
            collectionName: "canvases",
            data: canvas,
            key: name,
            transaction
        });
    }

    public async getCanvasNames() {
        return this.getAllKeys({collectionName: "canvases"});
    }

    public async getCanvasByName(name: string) {
        return this.get({collectionName: "canvases", key: name});
    }

    private async add(options: {
        collectionName: string,
        data: object,
        key: IDBValidKey,
        transaction?: IDBTransaction,
    }) {
        return await this.wrapTransaction({...options, operation: "add"})
    }

    private async get(options: {
        collectionName: string,
        key: IDBValidKey | IDBKeyRange,
        transaction?: IDBTransaction,
    }) {
        return await this.wrapTransaction({...options, operation: "get"});
    }

    private async getAllKeys(options: {
        collectionName: string,
        transaction?: IDBTransaction,
    }) {
        return await this.wrapTransaction({...options, operation: "getAllKeys"})
    }


    private wrapTransaction(args: {
        collectionName: string,
        operation: "add",
        data: object,
        key: IDBValidKey,
        transaction?: IDBTransaction,
    }): Promise<any>
    private wrapTransaction(args: {
        collectionName: string,
        operation: "get",
        key: IDBValidKey | IDBKeyRange,
        transaction?: IDBTransaction,
    }): Promise<any>
    private wrapTransaction(args: {
        collectionName: string,
        operation: "getAllKeys",
        transaction?: IDBTransaction,
    }): Promise<any>
    private wrapTransaction({collectionName, operation, data, key, transaction}: {
        collectionName: string,
        transaction?: IDBTransaction,
        operation: "add" | "get" | "getAllKeys",
        data?: object,
        key?: IDBValidKey | IDBKeyRange
    }): Promise<any> {
        if (!this.db) {
            throw new Error('Database is not opened yet');
        }
        return new Promise((resolve, reject) => {
            if (!transaction) {
                const mode = operation === "add" ? "readwrite" : "readonly";
                transaction = this.db.transaction(collectionName, mode);
            }

            let request;
            if (operation === "add") {
                request = transaction.objectStore(collectionName).add(data, key as IDBValidKey);
            } else {
                request = transaction.objectStore(collectionName)[operation](key as IDBValidKey | IDBKeyRange);
            }
            request.onsuccess = () => {
                resolve(request.result)
            };

            request.onerror = () => {
                reject(request.error)
            };
        })
    }
}