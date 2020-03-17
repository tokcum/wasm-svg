use crate::d3::Data;
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

        let canvas = web::SvgCanvas::new(width, height);
        let c = canvas._class().unwrap();
        body.append(&web::Element::from(canvas));

        Some(Doc {
            document: doc,
            canvas: body.select(".test2").unwrap(),
            data: None,
        })
    }
}
