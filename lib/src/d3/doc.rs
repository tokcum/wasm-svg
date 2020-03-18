use wasm_bindgen::JsCast;

use crate::d3::selector::Selector::*;
use crate::d3::Data;
use crate::d3::Selector;
use crate::web;

#[derive(Debug)]
pub struct Doc {
    pub document: web::Document,
    pub canvas: web::Element,
    data: Option<Data>,
}

impl Doc {
    pub fn new(width: f32, height: f32) -> Option<Doc> {
        let doc = web::Document::new().unwrap();

        // ToDo: check if body exists if not create it
        let body = doc.body().unwrap();
        body.append(&web::Element::create("p"));

        let canvas = web::SvgCanvas::new(width, height).unwrap();
        let c = canvas._class().unwrap();
        body.append(&web::Element::from(canvas));

        Some(Doc {
            document: doc,
            canvas: body.select(format!(".{}", c).as_str()).unwrap(),
            data: None,
        })
    }

    ///
    /// Selects all Elements by specified Selector
    ///
    pub fn select(&self, selector: Selector) -> Option<Vec<web::Element>> {
        let search = selector.to_string();
        let mut elements: Vec<web::Element> = Vec::new();

        match self
            .document
            .body()
            .unwrap()
            .n
            .query_selector_all(search.as_str())
        {
            Ok(list) => {
                let i = list.length();
                for i in 0..list.length() {
                    elements.push(web::Element::new(
                        list.item(i)
                            .unwrap()
                            .dyn_into::<web_sys::Element>()
                            .unwrap(),
                    ))
                }
            }
            Err(e) => {}
        }
        Some(elements)
    }
}
