use strong_xml::XmlRead;

#[derive(Debug, XmlRead)]
#[xml(tag = "document-info")]
pub struct KraDocumentInfo {
    #[xml(child = "about")]
    pub about: KraDocumentInfoAbout,

    #[xml(child = "author")]
    pub author: KraDocumentInfoAuthor,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "about")]
pub struct KraDocumentInfoAbout {
    #[xml(flatten_text = "title")]
    pub title: String,

    #[xml(flatten_text = "description")]
    pub description: String,

    #[xml(flatten_text = "subject")]
    pub subject: String,

    #[xml(flatten_text = "abstract")]
    pub r#abstract: String,

    #[xml(flatten_text = "keyword")]
    pub keyword: String,

    #[xml(flatten_text = "initial-creator")]
    pub initial_creator: String,

    #[xml(flatten_text = "editing-cycles")]
    pub editing_cycles: usize,

    #[xml(flatten_text = "editing-time")]
    pub editing_time: String,

    #[xml(flatten_text = "date")]
    pub date: String,

    #[xml(flatten_text = "creation-date")]
    pub creation_date: String,

    #[xml(flatten_text = "language")]
    pub language: String,

    #[xml(flatten_text = "license")]
    pub license: String,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "author")]
pub struct KraDocumentInfoAuthor {
    #[xml(flatten_text = "full-name")]
    pub full_name: String,

    #[xml(flatten_text = "creator-first-name")]
    pub creator_first_name: String,

    #[xml(flatten_text = "creator-last-name")]
    pub creator_last_name: String,

    #[xml(flatten_text = "initial")]
    pub initial: String,

    #[xml(flatten_text = "author-title")]
    pub author_title: String,

    #[xml(flatten_text = "position")]
    pub position: String,

    #[xml(flatten_text = "company")]
    pub company: String,
}
