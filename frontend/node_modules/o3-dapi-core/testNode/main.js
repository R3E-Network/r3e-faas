const o3dapi = require('../build/bundle.js');

const hex2strInput = '68656c6c6f';
const hex2strExpected = 'hello';
const hex2strResult = o3dapi.utils.hex2str(hex2strInput);
console.log('hex2str', hex2strExpected === hex2strResult);


const str2hexInput = 'hello';
const str2hexExpected = '68656c6c6f';
const str2hexResult = o3dapi.utils.str2hex(str2hexInput);
console.log('str2hex', str2hexExpected === str2hexResult);


const hex2intInput = '00e1f505';
const hex2intExpected = 100000000;
const hex2intResult = o3dapi.utils.hex2int(hex2intInput);
console.log('hex2int', hex2intExpected === hex2intResult);


const int2hexInput = 100000000;
const int2hexExpected = '00e1f505';
const int2hexResult = o3dapi.utils.int2hex(int2hexInput);
console.log('int2hex', int2hexExpected === int2hexResult);


const reverseHexInput = 'bc99b2a477e28581b2fd04249ba27599ebd736d3';
const reverseHexExpected = 'd336d7eb9975a29b2404fdb28185e277a4b299bc';
const reverseHexResult = o3dapi.utils.reverseHex(reverseHexInput);
console.log('reverseHex', reverseHexExpected === reverseHexResult);


const address2scriptHashInput = 'Ab2fvZdmnM4HwDgVbdBrbTLz1wK5TcEyhU';
const address2scriptHashExpected = 'd336d7eb9975a29b2404fdb28185e277a4b299bc';
const address2scriptHashResult = o3dapi.utils.address2scriptHash(address2scriptHashInput);
console.log('address2scriptHash', address2scriptHashExpected === address2scriptHashResult);


const scriptHash2addressInput = 'd336d7eb9975a29b2404fdb28185e277a4b299bc';
const scriptHash2addressExpected = 'Ab2fvZdmnM4HwDgVbdBrbTLz1wK5TcEyhU';
const scriptHash2addressResult = o3dapi.utils.scriptHash2address(scriptHash2addressInput);
console.log('scriptHash2address', scriptHash2addressExpected === scriptHash2addressResult);
