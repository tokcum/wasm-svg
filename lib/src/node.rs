pub struct Node(web_sys::Node);

impl Node {
    pub fn from(n: web_sys::Node) -> Node {
        Node(n)
    }
}
