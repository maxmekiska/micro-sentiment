#[path = "../src/sentiment_fn.rs"]
mod sentiment_fn;

use sentiment_fn::TextRequest;

#[test]
fn test_text_request() {
    let text = String::from("test text");
    let text_request = TextRequest { text };

    assert_eq!(text_request.text, "test text");
}
