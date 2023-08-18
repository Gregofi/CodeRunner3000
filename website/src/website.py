import os
from secrets import token_hex

from flask import Blueprint
from flask import render_template
from flask import Flask
from flask import request
from flask import url_for
from flask import redirect
from flask import abort
from flask import flash
from flask import make_response
from werkzeug.security import generate_password_hash, gen_salt, check_password_hash

import requests
import jwt

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
    response = requests.post("http://evaluator:7800/run", json=code)
    json = response.json()
    return json
