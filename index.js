import init, { run_app } from './pkg/web_yew.js';
async function main() {
   await init('/pkg/web_yew_bg.wasm');
   run_app();
}
main()
