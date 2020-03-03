use crate::web::Document;
use crate::web::Element;

pub struct SvgElement(pub web_sys::Element);

impl SvgElement {
    pub fn new(name: &str) -> Self {
        let doc = Document::new();
        SvgElement(
            doc.0.create_element_ns(Some("http://www.w3.org/2000/svg"), name)
                .unwrap(),
        )
    }

    pub fn id(&self) -> String {
        self.0.id()
    }

    pub fn set_id(&self, id: &str) -> &Self {
        self.0.set_id(id);
        self
    }
}
