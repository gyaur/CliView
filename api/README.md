# API description

All request and response bodies are empty unless specified otherwise

## Routes:

**/stream [POST]**
* Body: [json] `{"url": url}`
  * where url is either a youtube video or address of tcp server hosting a local file stream
* Responses:
  * 200 - OK
    * The video should start streaming interrupting any current content if neccessary
  * 400 - Bad Request
    * bad link or incorrect json
  * 500 - Internal server error


**/queue [POST]**
* Body: [json] `{"url": url}`
  * where url is either a youtube video or address of tcp server hosting a local file stream
* Responses:
  * 200 - OK
    * Video added to the queue
  * 400 - Bad Request
    * bad link or incorrect json
  * 500 - Internal server error


**/queue [GET]**
* Body: Empty
* Responses:
  * 200 - OK
    * [json] List of videos in the queue `{"queue": [{"url": url}],...}`
  * 400 - Bad Request
    * bad link or incorrect json
  * 500 - Internal server error

**/inc [POST]**
* Body: Empty
* Responses:
  * 200 - OK
    * Increases volume of playback by 1
  * 500 - Internal server error

**/dec [POST]**
* Body: Empty
* Responses:
  * 200 - OK
    * Decreases volume of playback by 1
  * 500 - Internal server error

**/volume [POST]**
* Body: [json] `{"volume": volume}`
  * where *volume* is in range [0,10]
* Responses:
  * 200 - OK
    * The video of the volume is set to *volume*
  * 400 - Bad Request
    * incorrect json or volume value
  * 500 - Internal server error


**/volume [GET]**
* Body: Empty
* Responses:
  * 200 - OK
    * [json] `{"volume": volume}`
    * where *volume* is the current volume in range [0,10] 
  * 400 - Bad Request
    * incorrect json
  * 500 - Internal server error

**/seek [POST]**
* Body: [json] `{"seek": ammount}`
  * where *ammount* is multiple of 30 or 600 and represented in seconds. Positive *ammount* is seek forward and negaitve *ammount* seek backwards
* Responses:
  * 200 - OK
    * seek happens
  * 400 - Bad Request
    * incorrect json or ammount
  * 500 - Internal server error


**/skip [POST]**
* Body: Empty
  * where *ammount* is multiple of 30 or 600 and represented in seconds. Positive *ammount* is seek forward and negaitve *ammount* seek backwards
* Responses:
  * 200 - OK
    * currently playing video is skipeed
  * 400 - Bad Request
    * incorrect json or ammount
  * 500 - Internal server error


## Dummy API

### Requirements:
* python 3.x
* flask


```
pip install flask
```

### Running:
```
python  dummy_api.py
```