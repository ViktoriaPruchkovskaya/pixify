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
        let db = await this.open();
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

    public async setCanvas(canvas: Canvas, name: string) {
        this.assertConnection();
        await this.wrapTransaction("canvases", "add", canvas, name);
    }

    public async getCanvasNames() {
        this.assertConnection();
        return this.wrapTransaction("canvases", "getAllKeys");
    }

    public async getCanvasByName(name: string) {
        this.assertConnection();
        return this.wrapTransaction("canvases", "get", name);
    }

    private assertConnection() {
        if (!this.db) {
            throw new Error('Database is not opened yet');
        }
    }


    private wrapTransaction(collectionName: string, operation: "add", data: any, key: IDBValidKey): Promise<any>
    private wrapTransaction(collectionName: string, operation: "get", data: IDBValidKey | IDBKeyRange): Promise<any>
    private wrapTransaction(collectionName: string, operation: "getAllKeys", data?: IDBValidKey | IDBKeyRange): Promise<any>
    private wrapTransaction(collectionName: string, operation: "add" | "get" | "getAllKeys", data?: any | IDBValidKey | IDBKeyRange, key?: IDBValidKey): Promise<any> {
        return new Promise((resolve, reject) => {
            const mode = operation === "add" ? "readwrite" : "readonly";
            const transaction: IDBTransaction = this.db.transaction(collectionName, mode);
            let request;
            if (operation === "add") {
                request = transaction.objectStore(collectionName).add(data, key);
            } else {
                request = transaction.objectStore(collectionName)[operation](data);
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