import os

from flask import Blueprint
from flask import render_template
from flask import Flask
from flask import request
from flask_sqlalchemy import SQLAlchemy

from markupsafe import escape

import requests

from werkzeug.middleware.proxy_fix import ProxyFix

from logging.config import dictConfig

import models

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

db = SQLAlchemy()

app = Flask(__name__)

db_uri = os.getenv("DB_URI")
if db_uri is None:
    print("DB_URI env variable not set")
    exit(1)

app.config["SQLALCHEMY_DATABASE_URI"] = os.getenv('DB_URI')

db.init_app(app)
with app.app_context():
    db.create_all()

app.wsgi_app = ProxyFix(
    app.wsgi_app, x_for=1, x_proto=1, x_host=1, x_prefix=1
)
os.getenv('DB_URI')


@app.route("/")
def hello_world():
    return render_template("website/index.html")


@app.route("/code")
def python_interpreter():
    return render_template("website/code.html")


@app.route("/blog")
def blog_all():
    return render_template("website/blog_all.html")


@app.route("/blog/<string:slug>")
def blog_view(slug: str):
    return render_template("website/blog.html", article_slug=escape(slug))


@app.route("/run-code", methods=["POST"])
def run_python():
    code = request.json
    app.logger.info("Received request for compilation, sending to server")
    response = requests.post("http://evaluator:7800/run", json=code)
    json = response.json()
    return json
