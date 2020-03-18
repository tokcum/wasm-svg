///
/// CSS Selector
///
///
///
///
/// https://www.w3schools.com/cssref/css_selectors.asp
///
pub enum Selector<'a> {
  Class(&'a str),
  ClassesAnd(Vec<&'a str>),
  ClassesOr(Vec<&'a str>),
  Id(&'a str),
  Search(&'a str),
}

impl<'a> Selector<'a> {
  pub(crate) fn to_string(&self) -> String {
    let mut search = String::from("");
    
    match self {
      Selector::Class(s) => search = [".", s].join(""),
      Selector::ClassesAnd(v) => {
        for s in v {
          search = [search, ".".to_string(), s.to_string()].concat();
        }
      }
      Selector::ClassesOr(v) => {
        if v.len() >= 1 {
          search = [".", v[0]].join("");
        }
        if v.len() >= 2 {
          for i in 1..v.len() {
            search = [search, ", .".to_string(), v[i].to_string()].join("");
          }
        }
      }
      Selector::Id(s) => search = ["#", s].join(""),
      Selector::Search(s) => search = String::from(*s),
    }
    search
  }
}

#[test]
fn test_class_to_string() {
let selector = Selector::Class("test");
  assert_eq!(selector.to_string(), ".test");
}

#[test]
fn test_classesand_to_string() {
  let selector = Selector::ClassesAnd(vec!["test"]);
  assert_eq!(selector.to_string(), ".test");
  
  let selector = Selector::ClassesAnd(vec!["test", "test2"]);
  assert_eq!(selector.to_string(), ".test.test2");
}

#[test]
fn test_classesor_to_string() {
  let selector = Selector::ClassesOr(vec!["test"]);
  assert_eq!(selector.to_string(), ".test");
  
  let selector = Selector::ClassesOr(vec!["test", "test2"]);
  assert_eq!(selector.to_string(), ".test, .test2");
}

#[test]
fn test_id_to_string() {
  let selector = Selector::Id("test");
  assert_eq!(selector.to_string(), "#test");
}

#[test]
fn test_search_to_string() {
  let selector = Selector::Search("test");
  assert_eq!(selector.to_string(), "test");
}
