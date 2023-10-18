use std::{fmt::Display, collections::HashMap, rc::Rc, cell::RefCell};

pub type Tag = Rc<RefCell<TagImpl>>;
#[derive(Clone,Debug)]
pub struct TagImpl{
    name:String,
    param : HashMap<Params,String>,
    childs : Childs,
    text : String,
}
impl TagImpl {
   pub fn add_child(&mut self,child:TagImpl){
       self.childs.childern.push(child);
   } 
   pub fn add_text(&mut self, t:&str){
       self.text = t.to_string()
   }
   pub fn add_id(&mut self,id:&str){
       self.param.insert(Params::Id, id.to_string());
   }
}


impl Display for TagImpl{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut child_str = "".to_string();
        if self.childs.len()==0{
            write!(f,"<{0}>{1}</{0}>",self.name,self.text)
        }else{
            self.childs.childern.iter().for_each(|i|{
                child_str.push_str(i.to_string().as_str());
            });
            write!(f,"<{0}>{1}{2}</{0}>",self.name,child_str,self.text)
        }
    }
}
pub struct TagBuilder{
    name:String,
    param : HashMap<Params,String>,
    childs : Vec<TagImpl> ,
    text : String,
}
impl TagBuilder{
    pub fn new(name:&str)-> TagBuilder{
        Self { name:name.to_string(),
        param: HashMap::new(),
        childs: Vec::new(),
        text: String::default() 
        }
    }
    pub fn add_id(&mut self,id:&str)->&mut Self{
        self.param.insert(Params::Id, id.to_string());
        self
    }
    pub fn build(&mut self)->Tag{
        Rc::new(
            RefCell::new(
                TagImpl { 
                    name: self.name.clone(),
                    param: self.param.clone(),
                    childs: Childs { childern: self.childs.clone() },
                    text: self.text.clone()
                    }
                )
            )
    }
}

#[derive(Hash,PartialEq,Clone,Debug)]
enum Params {
   Id 
}
impl Eq for Params {}

#[derive(Clone,Debug)]
struct Childs{
    childern : Vec<TagImpl>
}
impl Childs {
   fn len(&self)->usize{
        self.childern.len()
   }
}
