# Backend

## Installing on a raspberry pi:
```
curl -sL https://raw.githubusercontent.com/gyaur/CliView/master/api/install.sh | sudo sh

```


## API description

All request and response bodies are empty unless specified otherwise

## Routes:

**/stream [POST]**
* Body: [json] `{"url": url}`
  * where url is either a youtube video or address of http server hosting a local file stream
  * the http server should serve files realitve to its directry like the [python http.server](https://docs.python.org/3/library/http.server.html#http.server.SimpleHTTPRequestHandler.do_GET:~:text=The%20SimpleHTTPRequestHandler%20class%20can%20be%20used,files%20relative%20to%20the%20current%20directory%3A)
  * if streaming a local file the url format is http://\<ip>:\<port>/\<path_to_video>
* Responses:
  * 200 - OK
    * Interrupts any current content and starts playing the video. Does not start playback if nothing is playing.
  * 400 - Bad Request
    * bad link or incorrect json
  * 500 - Internal server error


**/queue [POST]**
* Body: [json] `{"url": url}`
  * where url is either a youtube video or address of http server hosting a local file stream
  * the http server should serve files realitve to its directry like the [python http.server](https://docs.python.org/3/library/http.server.html#http.server.SimpleHTTPRequestHandler.do_GET:~:text=The%20SimpleHTTPRequestHandler%20class%20can%20be%20used,files%20relative%20to%20the%20current%20directory%3A)
  * if streaming a local file the url format is http://\<ip>:\<port>/\<path_to_video>
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


**/play [POST]**
* Body: Empty
* Responses:
  * 200 - OK
    * Starts playback
  * 500 - Internal server error

**/pause [POST]**
* Body: Empty
* Responses:
  * 200 - OK
    * Starts playback
  * 500 - Internal server error


**/playback [GET]**
* Body: Empty
* Responses:
  * 200 - OK
    * [json] `{"status": playback_status}`
    * where playback status is true if a video is playing and false otherwise
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
* Body: [json] `{"ammount": ammount}`
  * where *ammount* is multiple of 30 or 600 and represented in seconds. Positive *ammount* is seek forward and negaitve *ammount* seek backwards
* Responses:
  * 200 - OK
    * seek happens
  * 400 - Bad Request
    * incorrect json or ammount
  * 500 - Internal server error


**/skip [POST]**
* Body: Empty
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

## Architecture

The api is split into 3 layers:
1 Proxy layer
2 Queueing, Command validation and preprocessing
3 Streaming layer
