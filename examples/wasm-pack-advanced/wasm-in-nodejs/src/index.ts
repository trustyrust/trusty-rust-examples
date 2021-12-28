import { data_processer } from "wasm-from-rust";

// function is a an example of how to handle errors from rust wasm
function rustWasmTest(dataForRust: any) {
    const { err, data } = data_processer(dataForRust)
    if (err) throw err
    return data
}


// Example of a successful result
(() => {
    try {
        const dataForRust = {
            js_num: 12,
            js_str: "abc",
            js_ary: [1, 2, 3, 4, 5]
        }
        const data = rustWasmTest(dataForRust)
        console.log(data)
    } catch (error) {
        console.error(error.message)
    }
})();

// Example of a Error begin returned
(() => {
    try {
        // This example will return a error because it is missing js_str
        const dataForRust = {
            js_num: 12,
            js_ary: [1, 2, 3, 4, 5]
        }
        const data = rustWasmTest(dataForRust)
        console.log(data)
    } catch (error) {
        console.error(error.message)
    }
})();
