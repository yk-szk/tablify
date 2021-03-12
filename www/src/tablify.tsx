import * as wasm from "tablify";
const arr = new Uint8Array(10);
try {
    const html = wasm.render("<p>template</p>", arr, "tmp.xlsx");
} catch(error) {
    console.error(error);
}

function T() {
    return (
        <p>a</p>
    )
}

export default T;