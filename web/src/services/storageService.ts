type Operation = 'add' | 'get' | 'getAllKeys' | 'clear';

type Mode = 'readwrite' | 'readonly';

export class StorageService {
    public readonly db: IDBDatabase;
    public static instance: StorageService;

    private constructor(db: IDBDatabase) {
        this.db = db;
    }

    static async getInstance() {
        if (StorageService.instance) {
            return StorageService.instance;
        }
        const db = await this.open();
        return (StorageService.instance = new StorageService(db));
    }

    private static async open(): Promise<IDBDatabase> {
        return new Promise((resolve, reject) => {
            const request = window.indexedDB.open('store', 1);

            request.onupgradeneeded = function () {
                const db = request.result;
                if (!db.objectStoreNames.contains('canvases')) {
                    db.createObjectStore('canvases');
                }
            };

            request.onerror = () => {
                reject('Cannot open storage');
            };

            request.onsuccess = (event: Event) => {
                resolve((event.target as IDBOpenDBRequest).result);
            };
        });
    }

    public async add<T>(options: {
        storeName: string;
        data: object;
        key: IDBValidKey;
        transaction?: IDBTransaction;
    }): Promise<T> {
        return await this.wrapTransaction({ ...options, operation: 'add' });
    }

    public async get<T>(options: {
        storeName: string;
        key: IDBValidKey | IDBKeyRange;
        transaction?: IDBTransaction;
    }): Promise<T> {
        return await this.wrapTransaction({ ...options, operation: 'get' });
    }

    public async getAllKeys<T>(options: {
        storeName: string;
        transaction?: IDBTransaction;
    }): Promise<T> {
        return await this.wrapTransaction({
            ...options,
            operation: 'getAllKeys',
        });
    }

    public async clear(options: {
        storeName: string;
        transaction?: IDBTransaction;
    }): Promise<void> {
        return await this.wrapTransaction({
            ...options,
            operation: 'clear',
        });
    }

    private wrapTransaction<T>(args: {
        storeName: string;
        operation: 'add';
        data: object;
        key: IDBValidKey;
        transaction?: IDBTransaction;
    }): Promise<T>;
    private wrapTransaction<T>(args: {
        storeName: string;
        operation: 'get';
        key: IDBValidKey | IDBKeyRange;
        transaction?: IDBTransaction;
    }): Promise<T>;
    private wrapTransaction<T>(args: {
        storeName: string;
        operation: 'getAllKeys' | 'clear';
        transaction?: IDBTransaction;
    }): Promise<T>;
    private wrapTransaction<T>({
        storeName,
        operation,
        data,
        key,
        transaction,
    }: {
        storeName: string;
        transaction?: IDBTransaction;
        operation: Operation;
        data?: object;
        key?: IDBValidKey | IDBKeyRange;
    }): Promise<T> {
        if (!this.db) {
            throw new Error('Database is not opened yet');
        }
        return new Promise((resolve, reject) => {
            if (!transaction) {
                const mode = this.getMode(operation);
                transaction = this.db.transaction(storeName, mode);
            }

            let request;
            switch (operation) {
                case 'add': {
                    request = transaction
                        .objectStore(storeName)
                        .add(data, key as IDBValidKey);
                    break;
                }
                case 'clear': {
                    request = transaction.objectStore(storeName).clear();
                    break;
                }
                default: {
                    request = transaction
                        .objectStore(storeName)
                        [operation](key as IDBValidKey | IDBKeyRange);
                }
            }

            request.onsuccess = () => {
                resolve(request.result);
            };

            request.onerror = () => {
                reject(request.error);
            };
        });
    }

    private getMode(operation: Operation): Mode {
        switch (operation) {
            case 'add':
            case 'clear':
                return 'readwrite';
            case 'getAllKeys':
            case 'get':
                return 'readonly';
        }
    }
}
