use custom_clone::{create_draft, Document};

#[test]
fn test_create_draft() {
    let doc = Document {
        title: String::from("My Report"),
        content: String::from("Some content here."),
    };
    let (original, draft) = create_draft(doc);
    assert_eq!(original.title, "My Report");
    assert_eq!(draft.title, "My Report (Draft)");
}

#[test]
fn test_content_preserved() {
    let doc = Document {
        title: String::from("Test"),
        content: String::from("Body text"),
    };
    let (original, draft) = create_draft(doc);
    assert_eq!(original.content, "Body text");
    assert_eq!(draft.content, "Body text");
}

#[test]
fn test_independence() {
    let doc = Document {
        title: String::from("Doc"),
        content: String::from("Content"),
    };
    let (original, draft) = create_draft(doc);
    assert_ne!(original.title, draft.title);
}
