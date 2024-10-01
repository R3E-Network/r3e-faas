// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

import { op_defer } from "ext:core/ops";
import { asString, uncurryThis } from "./infra.js";

function checkThis(thisArg) {
    if (thisArg !== null && thisArg !== undefined && thisArg !== globalThis) {
        throw new TypeError("Illegal invocation");
    }
}

const core = Deno.core;
const primordials = Deno.primordials;
const getAsyncContext = Deno.core.getAsyncContext;
const setAsyncContext = Deno.core.setAsyncContext;

/**
 * Call a callback function immediately.
 */
export function setImmediate(callback, ...args) {
    const asyncContext = getAsyncContext();
    return core.queueImmediate(() => {
        const oldContext = getAsyncContext();
        try {
            setAsyncContext(asyncContext);
            return Reflect.apply(callback, globalThis, args);
        } finally {
            setAsyncContext(oldContext);
        }
    });
}

/**
 * Call a callback function after a delay.
 */
export function setTimeout(callback, timeout = 0, ...args) {
    checkThis(this);
    // If callback is a string, replace it with a function that evals the string on every timeout
    if (typeof callback !== "function") {
        const unboundCallback = asString(callback);
        callback = () => primordials.indirectEval(unboundCallback);
    }

    const unboundCallback = callback;
    const asyncContext = getAsyncContext();
    callback = () => {
        const oldContext = getAsyncContext();
        try {
            setAsyncContext(asyncContext);
            Reflect.apply(unboundCallback, globalThis, args);
        } finally {
            setAsyncContext(oldContext);
        }
    };

    return core.queueUserTimer(
        core.getTimerDepth() + 1,
        false,
        timeout,
        callback,
    );
}

/**
 * Call a callback function after a delay.
 */
export function setInterval(callback, timeout = 0, ...args) {
    checkThis(this);
    if (typeof callback !== "function") {
        const unboundCallback = asString(callback);
        callback = () => primordials.indirectEval(unboundCallback);
    }
    const unboundCallback = callback;
    const asyncContext = getAsyncContext();
    callback = () => {
        const oldContext = getAsyncContext(asyncContext);
        try {
            setAsyncContext(asyncContext);
            Reflect.apply(unboundCallback, globalThis, args);
        } finally {
            setAsyncContext(oldContext);
        }
    };

    return core.queueUserTimer(
        core.getTimerDepth() + 1,
        true,
        timeout,
        callback,
    );
}

/**
 * Clear a timeout or interval.
 */
export function clearTimeout(id = 0) {
    checkThis(this);
    core.cancelTimer(id);
}

/**
 * Clear a timeout or interval.
 */
export function clearInterval(id = 0) {
    checkThis(this);
    core.cancelTimer(id);
}

// Defer to avoid starving the event loop. Not using queueMicrotask()
// for that reason: it lets promises make forward progress but can
// still starve other parts of the event loop.
export function defer(go) {
    const then = uncurryThis(Promise.prototype.then);
    then(op_defer(), () => go());
}
