const ffi = require('ffi-napi');

console.log('Node.js started');

try {
    const lib = ffi.Library('../nodewithrust/target/release/libnodewithrust', {
        'add': ['int', ['int', 'int']],
    });

    let result = lib.add(1, 2);
    console.log('Result:', result);
} catch (err) {
    console.error('Error:', err);
}
