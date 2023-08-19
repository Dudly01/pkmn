import init, * as wasm from './pkg/net.js';

const video = document.getElementById("video");
const canvas = document.getElementById("canvas");

const start_btn = document.getElementById("start_button");
const stop_btn = document.getElementById("stop_button");
const snapshot_btn = document.getElementById("snapshot_button");
const draw_btn = document.getElementById("draw_button");
const draw_wasm_btn = document.getElementById("draw_wasm_button");
const gameboy_btn = document.getElementById("gameboy_button");

const text_output = document.getElementById("output");
text_output.style.cssText =
    `
    font-family: monospace;
    white-space: pre-wrap;
    `;

var displayMediaOptions = {
    video: {
        cursor: "always",
    },
    audio: false,
};

start_btn.onclick = function (e) {
    startSharing();
};
stop_btn.onclick = function (e) {
    stopSharing();
};
snapshot_btn.onclick = function (e) {
    takeSnapshot();
};
draw_btn.onclick = function (e) {
    draw();
};
draw_wasm_btn.onclick = function (e) {
    drawWasm();
};
gameboy_btn.onclick = function (e) {
    gameboy();
};

async function startSharing() {
    try {
        video.srcObject = await navigator.mediaDevices.getDisplayMedia(
            displayMediaOptions
        );
    } catch (error) {
        console.log(error);
    }
}

function stopSharing() {
    let tracks = video.srcObject.getTracks();
    tracks.forEach((track) => track.stop());
    video.srcObject = null;
}

const takeSnapshot = () => {
    let target_width = video.videoWidth / 1;
    let target_height = video.videoHeight / 1;
    canvas.width = target_width;
    canvas.height = target_height;
    canvas.getContext('2d').drawImage(video, 0, 0, target_width, target_height);
    return 1;
};

// Draws green every second pixel of the image.
const draw = () => {
    let imageData = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height);
    let pixelData = imageData.data;

    // Iterate through the pixels of the RGBA image.
    for (let i = 0; i < pixelData.length; i += 4) {
        // Check if it's an even pixel (every second one)
        if ((i / 4) % 10 === 0) {
            pixelData[i] = 0;     // Red component
            pixelData[i + 1] = 255; // Green component
            pixelData[i + 2] = 0;   // Blue component
            // The alpha component (pixelData[i + 3]) remains unchanged
        }
    }

    // Update the modified pixel data back to the canvas
    canvas.getContext('2d').putImageData(imageData, 0, 0);
};

async function drawWasm() {
    // Instantiate the WebAssembly module
    await init();

    let imageData = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height);
    let pixelData = imageData.data;

    wasm.draw(pixelData);

    canvas.getContext('2d').putImageData(imageData, 0, 0);
}

async function gameboy() {
    // Instantiate the WebAssembly module
    await init();

    // Update canvas from video, use source size
    let target_width = video.videoWidth;
    let target_height = video.videoHeight;
    canvas.width = target_width;
    canvas.height = target_height;
    canvas.getContext('2d').drawImage(video, 0, 0, target_width, target_height);

    // Get ImageData of whole canvas
    let imageData = canvas.getContext('2d').getImageData(0, 0, target_width, target_height);
    let pixelData = imageData.data;

    // Try locating the GameBoy
    try {
        var results = wasm.read_stats_from_screen(pixelData, target_width, target_height);
        text_output.innerHTML = results;
    } catch (error) {
        text_output.textContent = error;
        return;
    }
}
