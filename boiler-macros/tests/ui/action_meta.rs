use boiler::actions::ActionMeta;
use boiler_macros::ActionMeta;

/// Generates GitHub Actions config for Rust projects
#[derive(ActionMeta)]
struct RustCiAction;

fn main() {
    let action = RustCiAction;
    assert_eq!(action.name(), "RustCi");
    assert_eq!(
        action.description(),
        "Generates GitHub Actions config for Rust projects"
    );
    assert_eq!(action.default_enabled(), true);
}
