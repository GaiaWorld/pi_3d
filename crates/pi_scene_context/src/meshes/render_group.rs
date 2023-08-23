

#[derive(Debug, Clone, Copy)]
pub struct RenderGroup(pub usize);
impl Default for RenderGroup {
    fn default() -> Self {
        Self(2000)
    }
}
