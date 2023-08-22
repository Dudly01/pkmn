import init, * as wasm from './pkg/net.js';

const video = document.getElementById("video");
const canvas = document.getElementById("canvas");

const button_start = document.getElementById("button_start");
const button_stop = document.getElementById("button_stop");
const button_scan = document.getElementById("button_scan");
const button_interval_scan = document.getElementById("button_interval_scan");

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

    // Try locating the GameBoy
    try {
        var results = wasm.read_stats_from_screen(pixelData, target_width, target_height);
        text_output.innerHTML = results;
    } catch (error) {
        text_output.textContent = error;
        return;
    }
}

function init_background_grid() {
    // Get the grid container and calculate its dimensions
    const gridContainer = document.getElementById('gridContainer');
    const gridItemSize = 100; // Adjust the size to match your grid item size
    const gridGap = 10; // Adjust the gap between grid items

    // Calculate the number of rows and columns based on window dimensions
    const windowWidth = window.innerWidth;
    const windowHeight = window.innerHeight;
    const availableWidth = windowWidth - gridGap; // Adjust for margins or padding
    const availableHeight = windowHeight - gridGap; // Adjust for margins or padding
    const columns = Math.floor(availableWidth / (gridItemSize + gridGap));
    const rows = Math.floor(availableHeight / (gridItemSize + gridGap));
    const numberOfItems = columns * rows;

    // https://bulbapedia.bulbagarden.net/wiki/Game_Boy_Color
    const colors = [
        "FF69B4",  // Strawberry
        "3E2F84",  // Grape
        "78C850",  // Kiwi
        "FFD733",  // Dandelion
        "008080",  // Teal
    ];

    // Create and fill the grid items
    for (let i = 0; i < numberOfItems; i++) {
        const gridItem = document.createElement('div');
        gridItem.classList.add('grid-item');

        // Apply random background image
        const randomColor = colors[Math.floor(Math.random() * colors.length)];
        gridItem.style.backgroundColor = `#${randomColor}`;

        // Append the grid item to the container
        gridContainer.appendChild(gridItem);
    }
}
init_background_grid();
