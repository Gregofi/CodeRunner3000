from wtforms import Form, StringField, PasswordField, validators

class LoginForm(Form):
    username = StringField('Username', [validators.Length(min=4, max=255)])
    password = PasswordField('Password', [
        validators.DataRequired(),
    ])
