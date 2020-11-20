
function playfunction() {
    let link = document.getElementById('inn').value;
    if(link==""){
      console.log("Paste something");
    }
    else{
        document.getElementById("label1").style.display = "none";
        document.getElementById("label2").style.display = "block";
        console.log("Your link has been started streaming: "+ link);
    }
}
function deletefunction() {
    document.getElementById('inn').value = "";
    document.getElementById("label2").style.display = "none";
    document.getElementById("label1").style.display = "block";
    console.log("Stream Ended");
}

function tabfunction() {
    window.open(document.getElementById('inn').value,"_blank");
}

function backfunction() {
    console.log("5 seconds back");
}

function frontfunction() {
    console.log("5 seconds forward");
}

function pausefunction() {
    document.getElementById("pause").style.display = "none";
    document.getElementById("repause").style.display = "inline-block";    
    console.log("Video Paused");
}
function repausefunction() {
    document.getElementById("repause").style.display = "none";
    document.getElementById("pause").style.display = "inline-block";
    console.log("Video Paused");
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