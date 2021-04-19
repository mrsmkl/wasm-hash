
const fs = require("fs")

async function main() {
	let a = await WebAssembly.instantiate(fs.readFileSync('pkg/wasm_hash_bg.wasm'))
	a.instance.exports.test()
}

main()

