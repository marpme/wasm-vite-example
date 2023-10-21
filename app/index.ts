// imports the freshly built wasm-pack dependency package
import { init, greet } from "../pkg";

const loadWASM = () => {
    // create dom initialization
    init()

    // say hello!
    greet()
}

loadWASM()
