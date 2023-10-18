use site::tag::TagBuilder;
use site::f;

fn main() {
    let v = vec!["Shakhtar","Dynamo","Zorya","Dnepr"];
    let html = TagBuilder::new("html").build();
    let head = TagBuilder::new("head").build();
    let body =TagBuilder::new("body").build();
    let div = TagBuilder::new("div").build();
    let h1 = TagBuilder::new("h1").build();
    let table = TagBuilder::new("table").build();
    v.iter().for_each(|i|{
        let td = TagBuilder::new("td").build();
        td.borrow_mut().add_text(i);
        let tr = TagBuilder::new("tr").build();
        f::add(&tr, &td);
        f::add(&table, &tr);
    });
    h1.borrow_mut().add_text("Life is shit!");
    f::add(&div,&table);
    f::add(&div,&h1);
    f::add(&body,&div);
    f::add(&html,&head);
    f::add(&html,&body);
    std::fs::write("index.html", html.borrow().to_string()).unwrap();
    //println!("{}",html.borrow());

}
