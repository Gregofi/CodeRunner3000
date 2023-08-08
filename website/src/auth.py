from flask import request
from flask import abort
from flask import current_app
import jwt
from models import User
from typing import Optional

JWT_SECRET = "8b079abb051d6b45dc23dc39acffc6d8688da240fbd8a7a7437966c36426d1a8"
JWT_COOKIE = "session"

def get_user(jwt_token: str) -> Optional[str]:
    if jwt_token is None:
        return None

    decoded = jwt.decode(jwt_token, JWT_SECRET, algorithms=['HS256'])

    if decoded is None:
        current_app.logger.warn(f"Invalid JWT token {jwt_token}")
        return None

    return decoded['username']


def admin(func):
    def check_admin():
        cookie_jwt = request.cookies.get(JWT_COOKIE)
        if cookie_jwt is None:
            return abort(401)

        username = get_user(cookie_jwt)
        if username is None:
            return abort(401)

        user = User.query.filter_by(username=username).first()
        if user is None:
            current_app.logger.warn(f"Got user {username} from JWT but couldn't find that username in DB")
            return abort(401)

        if user.is_admin == 1:
            return func()
        else:
            return abort(401)

    return check_admin
