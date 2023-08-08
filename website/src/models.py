from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()


class User(db.Model):
    __tablename__ = 'user'

    user_id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(255), unique=True, nullable=False)
    password = db.Column(db.String(255), nullable=False)
    salt = db.Column(db.String(255), nullable=False)
    email = db.Column(db.String(255), nullable=False)
    is_admin = db.Column(db.Integer)


class Article(db.Model):
    __tablename__ = 'article'

    article_id = db.Column(db.Integer, primary_key=True)
    heading = db.Column(db.String(255), unique=True, nullable=False)
    slug = db.Column(db.String(100), unique=True, nullable=False)
    text = db.Column(db.String)
    category_id = db.Column(db.Integer, db.ForeignKey('category.category_id'))

    category = db.relationship("Category", back_populates="articles")


class Category(db.Model):
    __tablename__ = 'category'

    category_id = db.Column(db.Integer, primary_key=True)
    parent_category_id = db.Column(db.Integer, db.ForeignKey('category.category_id'))
    name = db.Column(db.String(255), unique=True, nullable=False)

    children_categories = db.relationship(
                            "Category",
                            backref="parent_category",
                            remote_side=[category_id]
                        )
    articles = db.relationship("Article", back_populates="category")
