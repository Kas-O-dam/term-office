extern crate mairs;
//extern crate serde_xml_rs
extern crate console;

use console::Term;

fn main(){
    // [INIT]
    let size:(u16, u16) = Term::size(&Term);
    let mut canva:mairs::Field = mairs::Field{
        seq: Vec::new(),
        default_char: ' ',
    };
    let staticlay:mairs::Layer = mairs::Layer{
        x: size.0,
        y: size.1,
        context: Vec::new(),
    };
}
