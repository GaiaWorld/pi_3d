use pi_atom::Atom;


#[derive(Clone)]
pub enum EVaryingKind {
    GLPosition,
    GLFragColor,
}
impl EVaryingKind {
    pub fn code(&self) -> &str {
        match self {
            EVaryingKind::GLPosition => "gl_Position",
            EVaryingKind::GLFragColor => "gl_FragColor",
        }
    }
}

#[derive(Clone)]
pub struct Varying {
    pub format: Atom,
    pub name: Atom,
}
impl Varying {
    pub fn size(&self) -> usize {
        self.format.as_bytes().len() + self.name.as_bytes().len()
    }
    pub fn vs_code(&self, index: usize) -> String {
        let mut result = String::from("");
        result += "layout(location = ";
        result += index.to_string().as_str();
        result += ") out ";
        result += self.format.as_str();
        result += crate::prelude::S_SPACE;
        result += self.name.as_str();
        result += ";\r\n";

        result
    }
    pub fn fs_code(&self, index: usize) -> String {
        let mut result = String::from("");
        result += "layout(location = ";
        result += index.to_string().as_str();
        result += ") in ";
        result += self.format.as_str();
        result += crate::prelude::S_SPACE;
        result += self.name.as_str();
        result += ";\r\n";

        result
    }
}


#[derive(Clone, Default)]
pub struct Varyings(pub Vec<Varying>);
impl Varyings {
    pub fn size(&self) -> usize {
        let mut size = 0;
        self.0.iter().for_each(|item| {
            size += item.size();
        });

        size
    }
}

impl From<&pi_render::rhi::shader::ShaderVarying> for Varyings {
    fn from(value: &pi_render::rhi::shader::ShaderVarying) -> Self {
        let mut result = Varyings::default();
        value.0.iter().for_each(|val| {
            result.0.push(
                Varying {
                    format: val.format.clone(),
                    name: val.name.clone(),
                }
            )
        });
        result
    }
}

pub struct VaryingCode;
impl VaryingCode {
    pub fn vs_code(values: &Varyings) -> String {
        let mut result = String::from("");
        let mut index = 0;
        values.0.iter().for_each(|item| {
            result += item.vs_code(index).as_str();
            index += 1;
        });

        result
    }
    pub fn fs_code(values: &Varyings) -> String {
        let mut result = String::from("");
        let mut index = 0;
        values.0.iter().for_each(|item| {
            result += item.fs_code(index).as_str();
            index += 1;
        });

        result
    }
}

#[cfg(test)]
mod test {
    use pi_atom::Atom;

    use crate::shader::varying_code::{VaryingCode, Varyings, Varying};


    #[test]
    fn varying_code() {
        let attrs = Varyings(vec![
            Varying { 
                format: Atom::from(crate::static_string::S_VEC3),
                name: Atom::from(crate::static_string::S_V_POSITION),
            },
            Varying { 
                format: Atom::from(crate::static_string::S_VEC3),
                name: Atom::from(crate::static_string::S_V_NORMAL),
            },
        ]);

        println!("{}", VaryingCode::vs_code(&attrs));
        println!("{}", VaryingCode::fs_code(&attrs));
    }
}