import init, {
	apply,
} from "./wasm/pkg/wasm.js";

window.onload = async function () {
	await init();
	const canvas = document.createElement("CANVAS");
	const width = 10;
	const height = 10;
	canvas.width = width - (width % 2);
	canvas.height = height - (height % 4);

	const ctx = canvas.getContext("2d");
	ctx.fillStyle = "#ffffffff";
	ctx.fillRect(0, 0, canvas.width, canvas.height);

	const bytes = new Uint8Array(ctx.getImageData(0, 0, canvas.width, canvas.height).data.buffer);
	try {
		apply(bytes, canvas.width - 1, false, true, 1, false)
	} catch (e) {
		console.log(typeof e);
	}
};
