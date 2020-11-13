Designed by Emin Bayramov
Report for this week:


Manifest.json has been created and published. 
	Every extension has a JSON-formatted manifest file, named manifest.json, that provides important information.
	{
  "manifest_version": 2,

  "name": "Cliview",
  "description": "Streaming your Video",
  "version": "1.0",
  

  "browser_action": {
    "default_popup": "popup.html"
  },

  "background": {
    "scripts": ["eventPage.js"],
    "persistent": true
  },

  "permissions": [
            "contextMenus"
        ]
}

PopUp.html PopUp.css 
	has been created and published
	launch file, where our users will be able to stream link.

PopUp.js has been created
	Functionality works well
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
