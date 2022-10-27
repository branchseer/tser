use swc_common::Span;

#[derive(Debug)]
pub struct StructureError {
    pub message: Option<String>,
    pub span: Span,
}
impl From<Span> for StructureError {
    fn from(span: Span) -> Self {
        Self {
            message: None,
            span,
        }
    }
}
