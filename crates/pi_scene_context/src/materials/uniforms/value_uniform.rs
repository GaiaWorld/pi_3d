
use pi_atom::Atom;

#[derive(Debug)]
pub struct ValueBindDesc {
    pub binding: u32,
    pub mat4_name_list: Vec<Atom>,
    pub mat2_name_list: Vec<Atom>,
    pub vec4_name_list: Vec<Atom>,
    pub vec2_name_list: Vec<Atom>,
    pub float_name_list: Vec<Atom>,
    pub int_name_list: Vec<Atom>,
    pub uint_name_list: Vec<Atom>,
}
impl ValueBindDesc {
    pub fn label(&self) -> String {
        let mut result = String::from("");

        self.mat4_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });
        
        self.mat2_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.vec4_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.vec2_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.float_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.uint_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        result
    }
}