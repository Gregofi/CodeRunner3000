from secrets import token_hex

from flask import Blueprint
from flask import render_template
from flask import Flask
from flask import request

import requests
import os

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

# TODO
app.secret_key = token_hex(64)

# Nginx fix
app.wsgi_app = ProxyFix(
    app.wsgi_app, x_for=1, x_proto=1, x_host=1, x_prefix=1
)

EVALUATOR_ADDRESS = os.getenv("WEBSITE_EVALUATOR_URL", "http://evaluator:7800")


@app.route("/")
def index():
    return render_template("website/code.html")


@app.route("/code")
def python_interpreter():
    return render_template("website/code.html")


@app.route("/run-code", methods=["POST"])
def run_code():
    code = request.json
    app.logger.info(code)
    app.logger.info("Received request for compilation, sending to server")
    response = requests.post(f"{EVALUATOR_ADDRESS}/run", json=code)
    json = response.json()
    return json
