use std::{fmt::Display, collections::HashMap, rc::Rc, cell::RefCell};

use crate::f;

pub type TTag = Rc<RefCell<Tag>>;
pub struct Tag{
    name:String,
    param : HashMap<Params,String>,
    childs : Vec<TTag> ,
    text : String,
}
impl Tag{
    pub fn new(name:&str)->Rc<RefCell<Self>>{
        Rc::new(RefCell::new(
            Self { name:name.to_string(), 
                    childs: Vec::new(),
                    param:HashMap::new(),
                    text : String::new(),
            })) 
    }
    pub fn add_child(&mut self,c: TTag){
        self.childs.push(c);
    }
    pub fn set_id(&mut self,id:&str){
        self.param.insert(Params::Id, id.to_string());
    }
    pub fn set_text(&mut self,text:&str){
        self.text = text.to_string();
    }
}
impl Display for Tag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut childs_str = "".to_string();
      let mut param = "".to_string();
      self.param.iter().for_each(|(k,v)|{
          match k {
            Params::Id => param+= format!("id=\"{}\"",v.as_str()).as_str()
          }
      });
      self.childs.iter().for_each(|i|childs_str+=i.borrow().to_string().as_str());
       write!(f,"<{0} {2}>{1} {3}</{0}>", self.name, childs_str,param,self.text)
   } 
}
#[derive(Hash,PartialEq)]
enum Params {
   Id 
}
impl Eq for Params {}

