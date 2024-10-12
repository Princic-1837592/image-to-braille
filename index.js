import init, {
	parse,
} from "./wasm/pkg/wasm.js";

let INVERT, MONOSPACE, GRAY, CHARS, PIXELS, COPY;
let CHAR_COUNT;
let BRAILLE;
let LAST_IMAGE, LAST_CANVAS;
let FILE;

window.onload = async function () {
	await init();

	INVERT = document.getElementById("invert");
	MONOSPACE = document.getElementById("monospace");
	GRAY = document.getElementById("gray");
	CHARS = document.getElementById("width-chars");
	PIXELS = document.getElementById("width-pixels");
	COPY = document.getElementById("copy");
	CHAR_COUNT = document.getElementById("char-count");
	BRAILLE = document.getElementById("braille-art");
	FILE = document.getElementById("file");

	CHARS.addEventListener("change", updatePixels);
	PIXELS.addEventListener("change", updateChars);
	for (const input of [INVERT, MONOSPACE, GRAY, CHARS, PIXELS]) {
		input.addEventListener("change", async () => {
			await createCanvas(LAST_IMAGE);
		});
	}
	COPY.onclick = async () => {
		await navigator.clipboard.writeText(BRAILLE.innerText.trim());
	};

	document.body.ondragover = (e) => e.preventDefault();
	document.body.ondrop = async (e) => {
		e.preventDefault();
		await loadImage(URL.createObjectURL(e.dataTransfer.items[0].getAsFile()));
	}
	document.body.onpaste = async (e) => {
		e.preventDefault();
		await loadImage(URL.createObjectURL(e.clipboardData.items[0].getAsFile()));
	}
	FILE.onchange = async (e) => {
		await loadImage(URL.createObjectURL(e.target.files[0]));
	}

	await loadImage("title.png");
};

function convert() {
	const canvas = LAST_CANVAS;
	const ctx = canvas.getContext("2d");
	const bytes = new Uint8Array(ctx.getImageData(0, 0, canvas.width, canvas.height).data.buffer);
	try {
		const result = parse(
			bytes,
			canvas.width,
			INVERT.checked,
			true,
			GRAY.value,
			MONOSPACE.checked,
		).trim();
		document.getElementById("braille-art").innerText = result;
		CHAR_COUNT.innerText = result.length;
	} catch (e) {
		console.log(e);
	}
}

async function loadImage(image) {
	if (image === null || image === undefined) return;
	if (image !== LAST_IMAGE) {
		URL.revokeObjectURL(LAST_IMAGE);
		LAST_IMAGE = image;
	}
	await createCanvas(image);
}

async function createCanvas(src) {
	const canvas = document.createElement("canvas");
	const image = new Image();

	image.onload = () => {
		let width = image.width;
		let height = image.height;
		if (image.width !== PIXELS.value) {
			width = PIXELS.value;
			height = width * image.height / image.width;
		}
		canvas.width = width - (width % 2);
		canvas.height = height - (height % 4);
		const ctx = canvas.getContext("2d");
		ctx.fillStyle = "#ffffffff";
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.imageSmoothingEnabled = false;
		ctx.mozImageSmoothingEnabled = false;
		ctx.webkitImageSmoothingEnabled = false;
		ctx.msImageSmoothingEnabled = false;
		ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
		convert();
	}
	image.src = src;
	LAST_CANVAS = canvas;
}

function updatePixels() {
	PIXELS.value = CHARS.value * 2;
}

function updateChars() {
	CHARS.value = Math.floor(PIXELS.value / 2);
}
