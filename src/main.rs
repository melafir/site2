use site::tag::{Tag,TTag};
use site::f;
use std::{fs::File, io::Write};

fn main() {
    let v = vec!["Shakhtar","Dynamo","Zorya","Dnepr"];
    let html = Tag::new("html");
    let head = Tag::new("head");
    let body:TTag =Tag::new("body");
    let div = Tag::new("div");
    let h1 = Tag::new("h1");
    f::add(&html,&head);
    f::add(&html,&body);
    body.borrow_mut().add_child(div.clone());
    div.borrow_mut().set_id("container");
    div.borrow_mut().add_child(h1.clone());
    h1.borrow_mut().set_text("Life is shit!");
    let table = Tag::new("table");
    div.borrow_mut().add_child(table.clone());
    v.iter().for_each(|i|{
        let tr = Tag::new("tr");
        f::add(&table, &tr);
        let td = Tag::new("td");
        tr.borrow_mut().add_child(td.clone());
        td.borrow_mut().set_text(i);
    });
    let mut f = File::create("index.html").unwrap();
    write!(f,"{}",html.borrow()).unwrap();


    println!("{}",html.borrow());
}
