
use pi_render::renderer::attributes::EVertexDataKind;

#[derive(Clone)]
pub struct AttributeRefCode {
    pub format: String,
    pub name: String,
    pub kind: Option<EVertexDataKind>,
}
impl AttributeRefCode {
    pub fn size(&self) -> usize {
        self.format.as_bytes().len() + self.name.as_bytes().len() + 4
    }
    pub fn code(&self) -> String {
        let mut result = String::from("");

        result += self.format.as_str();
        result += crate::prelude::S_SPACE;
        result += self.name.as_str();
        if let Some(kind) = &self.kind {
            result += crate::prelude::S_EQUAL;
            result += kind.var_code();
        }
        result += ";"; result += crate::prelude::S_BREAK;

        result
    }
}

#[derive(Clone)]
pub struct AttributesRef(pub Vec<AttributeRefCode>);
impl AttributesRef {
    pub fn size(&self) -> usize {
        let mut size = 0;
        self.0.iter().for_each(|item| {
            size += item.size();
        });

        size
    }
}

pub struct VSBeginCode;
impl VSBeginCode {
    pub fn code(
        attrs: &AttributesRef
    ) -> String {
        let mut result = String::from("");
        attrs.0.iter().for_each(|attr| {
            result += attr.code().as_str();
        });

        result
    }
}


#[cfg(test)]
mod test {

    use pi_render::renderer::attributes::EVertexDataKind;
    use crate::shader::vs_begin_code::VSBeginCode;

    use super::{AttributeRefCode, AttributesRef};


    #[test]
    fn vs_begin_code() {
        let attrs = AttributesRef(vec![
            AttributeRefCode { 
                format: String::from(crate::static_string::S_VEC3),
                name: String::from(crate::static_string::S_POSITION),
                kind: Some(EVertexDataKind::Position),
            },
            AttributeRefCode { 
                format: String::from(crate::static_string::S_VEC3),
                name: String::from(crate::static_string::S_NORMAL),
                kind: Some(EVertexDataKind::Normal),
            },
        ]);

        println!("{}", VSBeginCode::code(&attrs));
    }
}