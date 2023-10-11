use crate::tag::Tag;
pub fn add(parent:&Tag,child:&Tag){
    parent.borrow_mut().add_child(child.borrow().to_owned());
}
