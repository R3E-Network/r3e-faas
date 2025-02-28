declare const Buffer: any;
import base58 from 'bs58';
import SHA256 from 'crypto-js/sha256';
import hexEncoding from 'crypto-js/enc-hex';

export function hex2str(input: string): string {
  return ab2str(hex2ab(input));
}

export function str2hex(input: string): string {
  return ab2hex(str2ab(input));
}

export function hex2int(input: string): number {
  return parseInt(reverseHex(input), 16);
}

export function int2hex(input: number): string {
  const h = Number(input).toString(16);
  return reverseHex(h.length % 2 ? '0' + h : h);
}

export function reverseHex(input: string): string {
  let out = '';
  for (let i = input.length - 2; i >= 0; i -= 2) {
    out += input.substr(i, 2);
  }
  return out;
}

export function address2scriptHash(input: string): string {
  const hash = ab2hex(base58.decode(input));
  return hash.substr(2, 40);
}

export function scriptHash2address(input: string): string {
  const ADDR_VERSION = '17';
  input = input.substr(0, 40);
  const firstSha = sha256(ADDR_VERSION + input);
  const secondSha = sha256(firstSha);
  const shaChecksum = secondSha.substr(0, 8);
  const arrayBuffer = hex2ab(ADDR_VERSION + input + shaChecksum);
  return base58.encode(new Buffer(arrayBuffer, 'hex'));
}

function sha256(data: string) {
  const hex = hexEncoding.parse(data);
  const sha = SHA256(hex).toString();
  return sha;
}

function ab2str(input: ArrayBuffer | ArrayLike<number>): string {
  return String.fromCharCode(...new Uint8Array(input));
}

function str2ab(input: string): Uint8Array {
  const result = new Uint8Array(input.length);
  for (let i = 0; i < input.length; i++) {
    result[i] = input.charCodeAt(i);
  }
  return result;
}

function ab2hex(input: ArrayBuffer | ArrayLike<number>): string {
  let result = '';
  const intArray = new Uint8Array(input);
  for (const i of intArray) {
    let str = i.toString(16);
    str = str.length === 0 ? '00' : str.length === 1 ? '0' + str : str;
    result += str;
  }
  return result;
}

function hex2ab(input: string): number[] {
  const result = [];
  while (input.length >= 2) {
    result.push(parseInt(input.substring(0, 2), 16));
    input = input.substring(2, input.length);
  }
  return result;
}
