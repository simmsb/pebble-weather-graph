import { configureTextEncoding } from "@bufbuild/protobuf/wire";

function encodeUtf8(text: string): Uint8Array<ArrayBuffer> {
    const encoded = encodeURIComponent(text);
    const bytes: number[] = [];
    for (let i = 0; i < encoded.length; i++) {
        if (encoded.charAt(i) === '%') {
            bytes.push(parseInt(encoded.substr(i + 1, 2), 16));
            i += 2;
        } else {
            bytes.push(encoded.charCodeAt(i));
        }
    }
    return new Uint8Array(bytes);
}

/** Throws on invalid UTF-8, whether or not strict decoding was requested. */
function decodeUtf8(bytes: Uint8Array): string {
    let escaped = '';
    for (let i = 0; i < bytes.length; i++) {
        escaped += (bytes[i] < 16 ? '%0' : '%') + bytes[i].toString(16);
    }
    return decodeURIComponent(escaped);
}

function checkUtf8(text: string): boolean {
    try {
        encodeURIComponent(text);
        return true;
    } catch (e) {
        return false;
    }
}

if (typeof TextEncoder === 'undefined' || typeof TextDecoder === 'undefined') {
    configureTextEncoding({ encodeUtf8, decodeUtf8, checkUtf8 });
}
