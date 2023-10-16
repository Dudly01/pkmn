import init, * as wasm from './pkg/net.js';

const video = document.getElementById("video");
const canvas = document.getElementById("canvas");

const button_start = document.getElementById("button_start");
const button_stop = document.getElementById("button_stop");
const button_scan = document.getElementById("button_scan");
const button_interval_scan = document.getElementById("button_interval_scan");
const button_canvas = document.getElementById("button_canvas");
const button_video = document.getElementById("button_video");

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
button_canvas.addEventListener('click', function () {
    canvas.classList.toggle('hidden');
    button_canvas.textContent = button_canvas.textContent === "Show snapshot" ? "Hide snapshot" : "Show snapshot";
});
button_video.addEventListener('click', function () {
    video.classList.toggle('hidden');  // The video.style.display is "" at first https://stackoverflow.com/a/44332288/12351436
    button_video.textContent = button_video.textContent === "Show screen" ? "Hide screen" : "Show screen";
});

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
