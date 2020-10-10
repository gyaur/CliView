import flask
from flask import jsonify

app = flask.Flask(__name__)
app.config["DEBUG"] = True    


@app.route('/stream', methods=['POST'])
@app.route('/queue', methods=['POST'])
@app.route('/inc', methods=['POST'])
@app.route('/dec', methods=['POST'])
@app.route('/volume', methods=['POST'])
@app.route('/seek', methods=['POST'])
@app.route('/skip', methods=['POST'])
def strem():
    return "", 200

@app.route('/queue', methods=['GET'])
def queue_get():
    return jsonify(
        {"queue": [{
            "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        }]})



@app.route('/volume', methods=['GET'])
def get_volume():
    return jsonify({"volume": 5})

app.run()