use pi_slotmap::DefaultKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PipelineKey {
    pub id: DefaultKey,
}