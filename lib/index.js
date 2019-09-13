var addon = require('../native');


const neighs = [
{ vector: [1, 4, 5], label: 'hi' }, 
{ vector: [3, 4, 7], label: 'hello' }
]

console.log(addon.euclideanKnn(5, neighs, [1, 2, 3]));
