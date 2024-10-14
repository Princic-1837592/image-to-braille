import init, {
	parse,
} from "./wasm/pkg/wasm.js";

let INVERT, MONOSPACE, GRAY, CHARS, PIXELS, COPY;
let THRESHOLD, THRESHOLD_VALUE;
let CHAR_COUNT;
let BRAILLE;
let LAST_IMAGE, LAST_CANVAS;
let FILE;
let CANNY_OPTIONS, CANNY_ACTIVE, CANNY_SIGMA, CANNY_LOW, CANNY_HIGH;

window.onload = async function () {
	await init();

	INVERT = document.getElementById("invert");
	MONOSPACE = document.getElementById("monospace");
	GRAY = document.getElementById("gray");
	THRESHOLD = document.getElementById("threshold");
	THRESHOLD_VALUE = document.getElementById("threshold-value");
	CHARS = document.getElementById("width-chars");
	PIXELS = document.getElementById("width-pixels");
	COPY = document.getElementById("copy");
	CHAR_COUNT = document.getElementById("char-count");
	BRAILLE = document.getElementById("braille-art");
	FILE = document.getElementById("file");
	CANNY_OPTIONS = document.getElementById("canny-options");
	CANNY_ACTIVE = document.getElementById("canny-active");
	CANNY_SIGMA = document.getElementById("canny-sigma");
	CANNY_LOW = document.getElementById("canny-low");
	CANNY_HIGH = document.getElementById("canny-high");

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
	for (const container of document.getElementsByClassName("threshold-container")) {
		const value = container.getElementsByClassName("threshold-value")[0];
		const input = container.getElementsByTagName("input")[0];
		input.oninput = async () => {
			value.innerText = input.value;
			await createCanvas(LAST_IMAGE);
		};
		value.innerText = input.value;
	}
	CANNY_ACTIVE.onchange = async () => {
		await createCanvas(LAST_IMAGE);
		for (const input of [CANNY_SIGMA, CANNY_LOW, CANNY_HIGH]) {
			input.disabled = !CANNY_ACTIVE.checked;
		}
		THRESHOLD.disabled = CANNY_ACTIVE.checked;
	}

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
	const ctx = canvas.getContext("2d", {alpha: false});
	const bytes = new Uint8Array(
		ctx
			.getImageData(0, 0, canvas.width, canvas.height)
			.data
			.filter((_, index) => (index + 1) % 4 !== 0)
			.buffer
	);
	try {
		const result = parse(
			bytes,
			canvas.width,
			INVERT.checked,
			GRAY.value,
			MONOSPACE.checked,
			THRESHOLD.value,
			CANNY_ACTIVE.checked ? CANNY_SIGMA.value / 10 : null,
			CANNY_ACTIVE.checked ? CANNY_LOW.value / 100 : null,
			CANNY_ACTIVE.checked ? CANNY_HIGH.value / 100 : null,
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
		ctx.fillStyle = "#ffffff";
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
