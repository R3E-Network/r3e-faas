export declare const AES128GCM = "aes-128-gcm";
export default class MessageEncryption {
    private ec;
    private key;
    private shared;
    private nonceMap;
    private cipherAlgorithm;
    getPublicKey(): any;
    setSharedKey(key: any): void;
    hasSharedKey(): boolean;
    encrypt(message: string): string | any;
    decrypt(input: string | any): {
        message: any;
        error?: string;
    };
    setCipherAlgorithOverride(algorithm: string): void;
    private _getSharedKey;
}
export declare function setEncryptionOverride({ randomFillSync, createCipheriv, createDecipheriv, createECDH, }: {
    randomFillSync: any;
    createCipheriv: any;
    createDecipheriv: any;
    createECDH: any;
}): void;
//# sourceMappingURL=messageEncryption.d.ts.map