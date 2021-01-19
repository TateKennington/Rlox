import * as wasm from "rlox";

document.getElementById("text-field").onchange = (e) => {
    console.log(wasm.run(e.target.value));
}