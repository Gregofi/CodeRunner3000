import os
from secrets import token_bytes

from flask import Blueprint
from flask import render_template
from flask import Flask
from flask import request
from flask import url_for
from flask import redirect
from flask import session
from flask import flash
from flask import make_response
from werkzeug.security import generate_password_hash, gen_salt, check_password_hash
# The flask command unfortunately runs the app one folder above.
from models import db, User, Article, Category
from forms import LoginForm

from markupsafe import escape

import requests
import jwt

from werkzeug.middleware.proxy_fix import ProxyFix

from logging.config import dictConfig

JWT_SECRET = "8b079abb051d6b45dc23dc39acffc6d8688da240fbd8a7a7437966c36426d1a8"
JWT_COOKIE = "session"

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

# Database initialization and loading models
db_uri = os.getenv("DB_URI")
if db_uri is None:
    print("DB_URI env variable not set")
    exit(1)

app.config["SQLALCHEMY_DATABASE_URI"] = db_uri

db.init_app(app)
with app.app_context():
    db.create_all()

# TODO: Temporary admin user
def create_test_admin():
    salt = gen_salt(32)
    hashed_password = generate_password_hash(salt + "password", method='sha256')
    new_user = User(username='admin', password=hashed_password,
                    salt=salt, email='filip.gregor98@gmail.com', is_admin=1)
    db.session.add(new_user)
    db.session.commit()
with app.app_context():
    create_test_admin()


# Nginx fix
app.wsgi_app = ProxyFix(
    app.wsgi_app, x_for=1, x_proto=1, x_host=1, x_prefix=1
)


def authenticated(cont):
    def check(*args, **kwargs):
        if session['session_token']:
            cont(*args, **kwargs)
        else:
            raise


@app.route("/")
def index():
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


@app.route("/admin")
def admin_view():
    return "Not implemented"


def verify_password(user: User, password: str) -> bool:
    return check_password_hash(user.password, user.salt + password)


@app.route("/auth", methods=["GET", "POST"])
def auth():
    form = LoginForm(request.form)
    if request.method == "POST" and form.validate():
        user = User.query.filter_by(username=form.username.data).first()
        if user is None or not verify_password(user, form.password.data):
            flash("Failed to login, check your username and password")
            return redirect(url_for('/auth'))

        jwt_token = jwt.encode({"username": user.username},
                               JWT_SECRET, algorithm="HS256")
        response = make_response(redirect(url_for('index')))
        response.set_cookie(JWT_COOKIE, jwt_token, httponly=True)
        return response
    else:
        return render_template("website/login.html", form=form)


@app.route("/run-code", methods=["POST"])
def run_python():
    code = request.json
    app.logger.info("Received request for compilation, sending to server")
    response = requests.post("http://evaluator:7800/run", json=code)
    json = response.json()
    return json
