var addon = require('../native');

function print(device, commands) {
  // Some arguments format stuff
  // ...
  console.log('Print done:', addon.print(device, commands));
}

var path = undefined;
var string = undefined;
var integer = undefined;

// Device information
var device = {path: string}; // File
var device = {host: string, port: integer}; // Network

// Print command + arguments list
var commands = [
  {name: "font", args: ["C"]},
  {name: "align", args: ["lt"]},
  {name: "style", args: ["bu"]},
  {name: "size", args: [0, 0]},
  {name: "text", args: ["The quick brown fox jumps over the lazy dog"]},
  {name: "text", args: ["敏捷的棕色狐狸跳过懒狗"]},
  {name: "barcode", args: ["12345678", "EAN8", "", "", 0, 0]},
  {name: "feed", args: [1]},
  {name: "cut", args: [false]},
];

// A test call
print(device, commands);
