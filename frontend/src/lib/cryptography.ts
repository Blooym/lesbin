enum EncryptionVersion {
    V1 = 'v1'
}

const ENCRYPTION_AES_ALGORITHM = 'AES-GCM';
const ENCRYPTION_AES_KEY_LENGTH = 256;
const ENCRYPTION_IV_LENGTH = 12;
const ENCRYPTION_VERSION = EncryptionVersion.V1;

// -- Encoding --

/**
 * Encode data into a Base64Url string.
 * @param data The data to encode.
 * @returns The Base64Url representation of the data.
 */
function _encodeBase64(data: Uint8Array<ArrayBuffer>): string {
    return btoa(String.fromCharCode(...data))
        .replace(/\+/g, '-')
        .replace(/\//g, '_');
}

/**
 * Decode a Base64Url string into a Uint8Array.
 * @param data The string to decode.
 * @returns A Uint8Array instance of the decoded Base64Url string.
 */
function _decodeBase64(data: string): Uint8Array {
    const base64 = data.replace(/-/g, '+').replace(/_/g, '/');
    return Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
}

// -- Key Management --

/**
 * Generates a new AES key using the Web Crypto API.
 *
 * @returns A promise that resolves to a new `CryptoKey` instance.
 * The key will be usable with the AES algorithm for encryption and decryption.
 */
export async function generateKey(): Promise<CryptoKey> {
    return await crypto.subtle.generateKey(
        { name: ENCRYPTION_AES_ALGORITHM, length: ENCRYPTION_AES_KEY_LENGTH },
        true,
        ['encrypt', 'decrypt']
    );
}

/**
 * Export a cryptographic key as a Base64 encoded string.
 *
 * @param key The `CryptoKey` to export.
 * @returns A promise that resolves to a Base64 encoded string
 * representing the raw key data.
 */
export async function exportKey(key: CryptoKey): Promise<string> {
    return _encodeBase64(new Uint8Array(await crypto.subtle.exportKey('raw', key)));
}

/**
 * Import a Base64 encoded key and convert it into a `CryptoKey` instance.
 *
 * @param key The Base64 encoded string representing the raw key to import.
 * @returns A promise that resolves to the imported `CryptoKey` instance.
 * The key will be associated with the specified cryptographic algorithm and will be usable
 * for encryption and decryption.
 */
export async function importKey(key: string): Promise<CryptoKey> {
    return await crypto.subtle.importKey(
        'raw',
        _decodeBase64(key),
        { name: ENCRYPTION_AES_ALGORITHM },
        true,
        ['encrypt', 'decrypt']
    );
}

// -- Encrypt & Decrypt --

/**
 * Encrypts the given data using the AES algorithm and a provided cryptographic key.
 *
 * This function generates a random initialization vector (IV), encrypts the provided
 * string data using the specified key, and returns the encrypted data along with the
 * IV and encryption version in a Base64 encoded format.
 *
 * @param data The data to encrypt.
 * @param key The `CryptoKey` to use for encryption.
 * @returns A promise that resolves to the encrypted data,
 * formatted as `{version}:ivBase64:encryptedDataBase64`.
 */
export async function encryptData(data: string, key: CryptoKey): Promise<string> {
    const iv = crypto.getRandomValues(new Uint8Array(ENCRYPTION_IV_LENGTH));
    const encrypted = await crypto.subtle.encrypt(
        { name: ENCRYPTION_AES_ALGORITHM, iv },
        key,
        new TextEncoder().encode(data)
    );
    return `${ENCRYPTION_VERSION}:${_encodeBase64(iv)}:${_encodeBase64(new Uint8Array(encrypted))}`;
}

/**
 * Decrypts the encrypted data using the AES algorithm and the provided cryptographic key.
 *
 * This function extracts the version, IV, and ciphertext from the provided Base64 encoded
 * string, and decrypts the data using the provided key. It returns the original plaintext data.
 *
 * @param encryptedData The Base64 encoded encrypted data in the format
 * `version:ivBase64:encryptedDataBase64`.
 * @param key The `CryptoKey` to use for decryption.
 * @returns A promise that resolves to the decrypted plaintext data.
 * @throws Throws an error if the encryption version is unsupported.
 */
export async function decryptData(encryptedData: string, key: CryptoKey): Promise<string> {
    const [version, ivBase64, ciphertextBase64] = encryptedData.split(':');
    switch (version) {
        case EncryptionVersion.V1: {
            return new TextDecoder().decode(
                await crypto.subtle.decrypt(
                    { name: ENCRYPTION_AES_ALGORITHM, iv: _decodeBase64(ivBase64) },
                    key,
                    _decodeBase64(ciphertextBase64)
                )
            );
        }
        default:
            throw new Error(`Unsupported encryption version: ${version}`);
    }
}
