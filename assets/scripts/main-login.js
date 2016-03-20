global.jQuery = require('jquery');
global.Tether = require('tether');
require("bootstrap");

console.log("ready7");

const gods = [
  {name: 'Douglas Crockfdddord'},
  {name: 'Guido van Rossum'},
  {name: 'Raffaele Esposito'}
];

let miracles = new Map();

miracles.set(gods[0], 'JavaScript2');
miracles.set(gods[1], 'Python');
miracles.set(gods[2], 'Pizza Margherita');

// Prints "JavaScript"
console.log(miracles.get(gods[0]));

console.log("tesffft");
