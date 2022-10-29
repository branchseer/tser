use swc_common::Span;

#[derive(Debug)]
pub struct StructureError {
    pub message: Option<String>,
    pub span: Span,
}
impl StructureError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: Some(message.into()),
        }
    }
}
impl From<Span> for StructureError {
    fn from(span: Span) -> Self {
        Self {
            message: None,
            span,
        }
    }
}
