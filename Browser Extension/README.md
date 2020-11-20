**Cliview Browser Extension**
Designed by Emin Bayramov

Manifest.json has been created and published. 
	Every extension has a JSON-formatted manifest file, named manifest.json, that provides important information.
```
{
  
  "manifest_version":2,
  "name":"Cliview Extension",
  "version":"2.0.1",
  "description":"This is a hybrid technology that streams your video to Tv",
  "author":"Emin Bayramov",
  "icons":{
    "128":"clogo.png"
  },
  
  "browser_action":{
    "default_popup":"popup.html",
    "default_icon":"clogo.png",
    "default_title":"Cliview Extension"

  },
  
  "permissions":["activeTab"],
  "content_scripts": [
    {
      "js": ["popup.js"],
      "css": ["popup.css"],
      "matches": ["http://*/*"] 
    }
  ]
}

```

PopUp.html PopUp.css 
	has been created and published
	launch file, where our users will be able to stream link.

PopUp.js has been created
	Functionality works well
```
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
```

**New Updates:**
	*Chrome does not allow inline style addition such as onlick functions, and gives below error
	*Refused to apply inline style because it violates the following Content Security Policy directive: "default-src 'self'". Note that 	'style-src' was not explicitly set, so 'default-src' is used as a fallback.
	*Due to this error, EventListener method has been used, 
```
document.addEventListener('DOMContentLoaded', function () {
  document.querySelector("#play").addEventListener('click', playfunction);
  document.querySelector("#delete").addEventListener('click', deletefunction);
  document.querySelector("#tab").addEventListener('click', tabfunction);
  document.querySelector("#back").addEventListener('click', backfunction);
  document.querySelector("#front").addEventListener('click', frontfunction);
  document.querySelector("#pause").addEventListener('click', pausefunction);
  document.querySelector("#repause").addEventListener('click', repausefunction);
});
```
