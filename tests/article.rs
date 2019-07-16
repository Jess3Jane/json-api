use json_api::{
    Document, 
    Attributes, 
    ResourceObject, 
    OptionalVec, 
    Relationships, 
    Links, 
    Link, 
    Relationship,
    GenericObject,
};
use serde_derive::{Serialize, Deserialize};
use serde_json::{self, json};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Article {
    title: String,
}
impl Attributes for Article { 
    fn kind() -> String { "articles".into() } 
    fn links(id: &str) -> Option<Links> {
        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/articles/{}", id)));
        Some(links)
    }
}
impl Article {
    fn add_author(article: &mut ResourceObject<Article>, author: &ResourceObject<People>) {
        let mut links = Links::new();
        links.insert("self".into(), 
            Link::Url(format!("/articles/{}/relationships/author", article.id)));
        links.insert("related".into(), Link::Url(format!("/articles/{}/author", article.id)));
        let r = Relationship {
            links: Some(links),
            data: OptionalVec::One(Some(author.into())),
            ..Default::default()
        };
        article.add_relationship("author".into(), r);
    }

    fn add_comments(article: &mut ResourceObject<Article>, comments: &[ResourceObject<Comment>]) {
        let mut links = Links::new();
        links.insert("self".into(),
            Link::Url(format!("/articles/{}/relationships/comments", article.id)));
        links.insert("related".into(), Link::Url(format!("/articles/{}/comments", article.id)));
        let r = Relationship {
            links: Some(links),
            data: OptionalVec::Many(comments.iter().map(|c| c.into()).collect()),
            ..Default::default()
        };
        article.add_relationship("comments".into(), r);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct People {
    first_name: String,
    last_name: String,
    contact: String,
}
impl Attributes for People { 
    fn kind() -> String { "people".into() } 
    fn links(id: &str) -> Option<Links> {
        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/people/{}", id)));
        Some(links)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Comment {
    body: String,
}
impl Attributes for Comment { 
    fn kind() -> String { "comments".into() } 
    fn links(id: &str) -> Option<Links> {
        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/comments/{}", id)));
        Some(links)
    }
}
impl Comment {
    fn add_author(comment: &mut ResourceObject<Comment>, author: &ResourceObject<People>) {
        let relationship = Relationship {
            data: OptionalVec::One(Some(author.into())),
            ..Default::default()
        };
        comment.add_relationship("author".into(), relationship);
    }
}

#[test]
fn test() {
    let author = ResourceObject::new("9".into(), Some(People { 
        first_name: "kitty".into(),
        last_name: "cat".into(),
        contact: "kitty@cat.space".into(), 
    }));
    let person_2 = ResourceObject::<People>::new("2".into(), None);

    let mut comment_5 = ResourceObject::new("5".into(), Some(Comment {
        body: "First!".into(),
    }));
    Comment::add_author(&mut comment_5, &person_2);
    let mut comment_12 = ResourceObject::new("12".into(), Some(Comment {
        body: "I like XML better".into(),
    }));
    Comment::add_author(&mut comment_12, &author);
    
    let comments = [comment_5, comment_12];

    let mut article = ResourceObject::new("1".into(), Some(Article {
        title: "JSON:API is kind of strange in rust".into(),
    }));
    Article::add_author(&mut article, &author);
    Article::add_comments(&mut article, &comments);

    let mut included = Vec::new();
    included.extend(comments.into_iter().map(|v| GenericObject::from(v)));
    included.push(author.into());

    let document = Document {
        data: OptionalVec::One(Some(article.into())),
        included: Some(included),
        ..Default::default()
    };

    let v1 = serde_json::to_value(&document).unwrap();
    let v2 : serde_json::Value = serde_json::from_str(include_str!("article.json")).unwrap();
    assert_eq!(v1, v2);
}
