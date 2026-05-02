/// A document with a title and content.
/// TODO: Add #[derive(Debug, Clone, PartialEq)] to enable cloning.
pub struct Document {
    pub title: String,
    pub content: String,
}

/// Creates a draft copy of a document with " (Draft)" appended to the title.
/// TODO: Clone the document, modify the clone's title, return both.
pub fn create_draft(doc: Document) -> (Document, Document) {
    todo!()
}
