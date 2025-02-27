// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

import {
    setImmediate,
    setTimeout,
    setInterval,
    clearTimeout,
    clearInterval,
    defer,
} from "./time.js";

import {
    base64Encode,
    base64Decode,
} from "./encoding.js";

import { Neo } from "./neo.js";

globalThis.setImmediate = setImmediate;
globalThis.setTimeout = setTimeout;
globalThis.setInterval = setInterval;
globalThis.clearTimeout = clearTimeout;
globalThis.clearInterval = clearInterval;

if (!globalThis.r3e) {
    globalThis.r3e = {};
}

Object.assign(globalThis.r3e, {
    defer,
    base64Encode,
    base64Decode,
    Neo,
});
