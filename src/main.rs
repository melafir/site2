use site::tag::TagBuilder;
use site::f;
use std::{fs::File, io::Write};

fn main() {
    let v = vec!["Shakhtar","Dynamo","Zorya","Dnepr"];
    let html = TagBuilder::new("html").build();
    let head = TagBuilder::new("head").build();
    let body =TagBuilder::new("body").build();
    let div = TagBuilder::new("div").build();
    let h1 = TagBuilder::new("h1").build();
    h1.borrow_mut().add_text("Life is shit!");
    f::add(&html,&head);
    f::add(&html,&body);
    f::add(&body,&div);
    f::add(&div,&h1);
    let table = TagBuilder::new("table").build();
    f::add(&div,&table);
    v.iter().for_each(|i|{
        let tr = TagBuilder::new("tr").build();
        f::add(&table, &tr);
        let td = TagBuilder::new("td").build();
        td.borrow_mut().add_text(i);
        f::add(&tr, &td);
    });
    let mut f = File::create("index.html").unwrap();
    write!(f,"{}",html.borrow()).unwrap();

    println!("{}",html.borrow());
}
