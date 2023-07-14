function main() {
    const video = document.getElementById("video");
    const canvas = document.getElementById("canvas");

    const start_btn = document.getElementById("start_button");
    const stop_btn = document.getElementById("stop_button");
    const snapshot_btn = document.getElementById("snapshot_button");

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
        let target_width = video.videoWidth / 2;
        let target_height = video.videoHeight / 2;
        canvas.width = target_width;
        canvas.height = target_height;
        canvas.getContext('2d').drawImage(video, 0, 0, target_width, target_height);
        return 1;
    };
}

main();
