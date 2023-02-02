fn main() {
    let mut parser = Parser::default();
    // std::fs::write("build_error.text", "zzzzzzz").unwrap();

    let r = parser
        .push_gen_path(&["src/shaders/"])
        .push_program(vec![ProgramDesc::new("src/shaders/test.vert", "src/shaders/test.frag", "src/shaders/aa")])
        .parse();
    match r {
        Ok(r) => {
            for shader in r.shader_result.iter() {
                std::fs::write(&shader.0, &shader.1).unwrap();
            }
        }
        Err(e) => {
            panic!("e============={:?}", e);
        }
    }
}