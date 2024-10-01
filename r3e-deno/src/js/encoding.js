// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


export function base64Encode(input) {
    return Buffer.from(input).toString("base64");
}

export function base64Decode(input) {
    return Buffer.from(input, "base64").toString("utf-8");
}



