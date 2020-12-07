import init, { run_app } from './wasm/web_yew.js';
async function main() {
   await init('/wasm/web_yew_bg.wasm');
   run_app();
}
main()
