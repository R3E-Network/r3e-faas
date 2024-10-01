// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

export function asString(value, nullAsEmpty = false) {
    if (typeof value === "string") {
        return value;
    } else if (value === null && nullAsEmpty) {
        return "";
    } else if (typeof value === "symbol") {
        throw new TypeError("symbol cannot be converted to a string");
    }

    return String(value);
}

export function uncurryThis(fn) {
    return function () { return fn.call.apply(fn, arguments); };
}

// export function requiredArguments(length, required, prefix) {
//     if (length < required) {
//         const msg = `${prefix ? prefix + ": " : ""}${required} argument${required === 1 ? "" : "s"} required, but only ${length} present`;
//         throw new TypeError(msg);
//     }
// }
