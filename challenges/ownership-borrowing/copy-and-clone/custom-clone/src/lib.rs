/// A document with a title and content.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub title: String,
    pub content: String,
}

/// Creates a draft copy of a document with " (Draft)" appended to the title.
pub fn create_draft(doc: Document) -> (Document, Document) {
    let mut draft = doc.clone();
    draft.title.push_str(" (Draft)");
    (doc, draft)
}
