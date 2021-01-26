import * as wasm from "rlox";

document.getElementById("text-field").onchange = () => {
    document.getElementById("run").click();
}

document.getElementById("run").onclick = (e) => {
    const result = document.createElement('p');
    const source = document.getElementById("text-field").value;
    if (source.length === 0) {
        return;
    }
    result.innerHTML = wasm.run(source);
    document.getElementById("results").appendChild(result);
}