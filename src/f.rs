use crate::tag::TTag;
pub fn add(parent:&TTag,child:&TTag){
    parent.borrow_mut().add_child(child.clone());
}
