import { add_nums, format_string } from "wasm-from-rust";

(() => {
    const num = add_nums(2, 14)
    console.log('add_nums result from wasm: %d', num)

    const str = format_string("wasm test")
    console.log('format_string result from wasm: %s', str)
})()
