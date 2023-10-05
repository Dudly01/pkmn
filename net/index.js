import init, * as wasm from './pkg/net.js';

const video = document.getElementById("video");
const canvas = document.getElementById("canvas");

const button_start = document.getElementById("button_start");
const button_stop = document.getElementById("button_stop");
const button_scan = document.getElementById("button_scan");
const button_interval_scan = document.getElementById("button_interval_scan");

const text_output = document.getElementById("output");

var displayMediaOptions = {
    video: {
        cursor: "always",
    },
    audio: false,
};

button_start.onclick = function (e) {
    startSharing();
};
button_stop.onclick = function (e) {
    stopSharing();
};
button_scan.onclick = function (e) {
    gameboy();
};
button_interval_scan.onclick = function (e) {
    periodic_gameboy();
};

let intervalId;

// Function to toggle the periodic calling
function periodic_gameboy() {
    if (intervalId) {
        // If intervalId is set, clear the interval and reset the variable
        clearInterval(intervalId);
        intervalId = undefined;
        button_interval_scan.textContent = "Start scanning";
    } else {
        // If intervalId is not set, start the interval and store the interval ID
        intervalId = setInterval(gameboy, 1000); // Call every 1000 milliseconds (1 second)
        button_interval_scan.textContent = "Stop scanning";
    }
}

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

    const t0 = performance.now();

    // Try locating the GameBoy
    try {
        var results = wasm.read_stats_from_screen(pixelData, target_width, target_height);
        text_output.innerHTML = results;
    } catch (error) {
        text_output.textContent = error;
    }

    const t1 = performance.now();
    console.log(`Scanning took ${t1 - t0} ms.`);
}
