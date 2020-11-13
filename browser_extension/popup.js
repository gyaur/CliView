function playfunction() {
    let link = document.getElementById('inn').value;
    if(link==""){
        alert("Paste something");
    }
    else{


    document.getElementById("label1").style.display = "none";
    document.getElementById("label2").style.display = "block";
    alert("Your link has been started streaming: "+ link);
}
}
function deletefunction() {
    document.getElementById('inn').value = "";
    document.getElementById("label2").style.display = "none";
    document.getElementById("label1").style.display = "block";
    alert("Stream Ended");
}

function tabfunction() {
    window.open(document.getElementById('inn').value,"_blank");
}

function backfunction() {
    alert("5 seconds back");
}

function frontfunction() {
    alert("5 seconds forward");
}

function pausefunction() {
    document.getElementById("pause").style.display = "none";
    document.getElementById("repause").style.display = "inline-block";    
    alert("Video Paused");
}
function repausefunction() {
    document.getElementById("repause").style.display = "none";
    document.getElementById("pause").style.display = "inline-block";
    alert("Video Paused");
}