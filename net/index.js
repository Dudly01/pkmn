import init, * as wasm from './pkg/net.js';

const video = document.getElementById("video");
const canvas = document.getElementById("canvas");

const start_btn = document.getElementById("button_start");
const stop_btn = document.getElementById("button_stop");
const gameboy_btn = document.getElementById("button_gameboy");

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
