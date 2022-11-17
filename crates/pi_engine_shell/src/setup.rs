use crate::engine_shell::EnginShell;

pub type SetupFn = dyn Fn(&EnginShell);

pub trait TSetup {
    fn setup(
        &mut self,
        engine: &EnginShell,
    );
}