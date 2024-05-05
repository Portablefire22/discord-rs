use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: Option<EmbedType>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<usize>,
    pub footer: Option<Footer>,
    pub image: Option<Image>,
    pub thumbnail: Option<Thumbnail>,
    pub video: Option<Video>,
    pub provider: Option<Provider>,
    pub author: Option<Author>,
    pub fields: Option<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum EmbedType {
    rich,
    image,
    video,
    gifv,
    article,
    link,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Footer {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Image {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Video {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
