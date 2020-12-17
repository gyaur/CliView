
function playfunction() {
    let link = document.getElementById('inn').value;
    if (link == "") {
        console.log("Paste something");
    }
    else {
        document.getElementById("label1").style.display = "none";
        document.getElementById("label2").style.display = "block";
        sendUrl(link);
    }
}
function deletefunction() {
    document.getElementById('inn').value = "";
    document.getElementById("label2").style.display = "none";
    document.getElementById("label1").style.display = "block";
    control("skip");
}

function tabfunction() {
    control("skip")
    setTimeout(() => window.open(document.getElementById('inn').value, "_blank"), 500)

}

function backfunction() {
    control("dec");
}

function frontfunction() {
    control("inc");
}

function pausefunction() {
    document.getElementById("pause").style.display = "none";
    document.getElementById("repause").style.display = "inline-block";
    control("pause");
}
function repausefunction() {
    document.getElementById("repause").style.display = "none";
    document.getElementById("pause").style.display = "inline-block";
    control("play");
}


document.addEventListener('DOMContentLoaded', function () {
    document.querySelector("#play").addEventListener('click', playfunction);
    document.querySelector("#delete").addEventListener('click', deletefunction);
    document.querySelector("#tab").addEventListener('click', tabfunction);
    document.querySelector("#back").addEventListener('click', backfunction);
    document.querySelector("#front").addEventListener('click', frontfunction);
    document.querySelector("#pause").addEventListener('click', pausefunction);
    document.querySelector("#repause").addEventListener('click', repausefunction);
});

function sendUrl(link) {
    let data = { url: link };

    fetch("http://raspberrypi.local:5000/queue", {
        method: "POST",
        body: JSON.stringify(data)
    }).then(res => {
        if (res.status == 200) {
            console.log("Link has been sent!");
        } else if (res.status == 400) {
            console.log("Link is not correct!");
        } else if (res.status == 500) {
            console.log("Internal server error");
        } else {
            console.log("There is something went wrong");
        }
    });
}

function control(type) {
    let url = "http://raspberrypi.local:5000/" + type;

    fetch(url, {
        method: "POST",
    }).then(res => {
        if (res.status == 200) {
            console.log("Request complete!");
        } else if (res.status == 500) {
            console.log("Internal server error");
        } else {
            console.log("There is something went wrong");
        }
    });
}
