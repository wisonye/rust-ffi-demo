const ffi = require('ffi-napi');

// console.log(`ffi: `, ffi)

const ffiTypesKeys = Object.keys(ffi.types)

console.log(`ffiTypes: `)

ffiTypesKeys.forEach(key => {
    const separator = key.length <= 8 ?`\t--> ` : `--> `

    console.log(`key: `, key, separator, ffi.types[key].ffi_type)
})
