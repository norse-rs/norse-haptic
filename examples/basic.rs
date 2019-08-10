use norse_haptic as hpt;

fn main() -> Result<(), Box<std::error::Error>> {
    unsafe {
        let mut instance = hpt::Instance::create()?;
        let devices = instance.enumerate_physical_devices()?;
        dbg!(devices.len());

        let system = instance.create_system()?;
        let mut session = instance.create_session(&system)?;

        let game_set = instance.create_action_set("game")?;
        let forward = game_set.create_action("forward", hpt::ActionType::BooleanInput, &[])?;
        let forward_path = instance.string_to_path("/user/mouse/input/left/click");

        let profile_path = instance.string_to_path("/interaction_profiles/norse/desktop");
        instance.suggest_interaction_profile_bindings(
            profile_path,
            &[hpt::ActionSuggestedBinding {
                action: forward,
                binding: forward_path,
            }],
        );

        session.attach_action_sets(&[game_set]);

        loop {
            session.sync_actions(&[game_set]);
        }
    }
    Ok(())
}
