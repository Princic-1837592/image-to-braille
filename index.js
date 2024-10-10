import init, {
	// apply,
} from "./wasm/pkg/wasm.js";

window.onload = async function () {
	await init();
};
