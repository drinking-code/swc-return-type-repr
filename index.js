const swc = require('@swc/core');
const path = require('path');

console.log('Return type seen by plugin:')

swc.transformFileSync('Test.ts', {
    jsc: {
        parser: {
            syntax: 'typescript',
        },
        experimental: {
            plugins: [[path.resolve('./target/wasm32-wasi/debug/swc_return_type_repr.wasm'), {}]]
        }
    }
});

console.log('Return type from parse function (actual):')

swc.parseFile('Test.ts', {
    syntax: 'typescript',
}).then(result => {
    console.log(result.body[0].body[0].function.returnType)
});
