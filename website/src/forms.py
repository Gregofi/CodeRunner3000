from wtforms import Form, StringField, PasswordField, TextAreaField, validators

class LoginForm(Form):
    username = StringField('Username', [validators.Length(min=4, max=255)])
    password = PasswordField('Password', [
        validators.DataRequired(),
    ])


class ArticleForm(Form):
    heading = StringField('Heading', [validators.Length(min=3, max=255)])
    slug = StringField('Slug', [validators.Length(min=3, max=100)])
    text = TextAreaField('Text', [validators.DataRequired()])
