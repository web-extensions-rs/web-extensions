import init, { start } from './pkg/new_tab_search.js';

console.log("WASM module loaded")

async function run() {
    await init();

    console.log("WASM module initialized")

    start();
}

run();
