from flask import Blueprint
from flask import render_template
from flask import Flask
from flask import request

import requests

from werkzeug.middleware.proxy_fix import ProxyFix

from logging.config import dictConfig

dictConfig({
    'version': 1,
    'formatters': {'default': {
        'format': '[%(asctime)s] %(levelname)s in %(module)s: %(message)s',
    }},
    'handlers': {'wsgi': {
        'class': 'logging.StreamHandler',
        'stream': 'ext://flask.logging.wsgi_errors_stream',
        'formatter': 'default'
    }},
    'root': {
        'level': 'INFO',
        'handlers': ['wsgi']
    }
})

bp = Blueprint("website", __name__)

app = Flask(__name__)

app.wsgi_app = ProxyFix(
    app.wsgi_app, x_for=1, x_proto=1, x_host=1, x_prefix=1
)


@app.route("/")
def hello_world():
    return render_template("website/index.html")


@app.route("/code")
def python_interpreter():
    return render_template("website/code.html")


@app.route("/run-code", methods=["POST"])
def run_python():
    code = request.json
    app.logger.info("Received request for compilation, sending to server")
    response = requests.post("http://runtime:7000/run", json=code)
    json = response.json()
    return json
