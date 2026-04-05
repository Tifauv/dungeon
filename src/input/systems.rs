use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;


pub fn disable_mouse(
    mut p_action_sources: ResMut<ActionSources>,
    p_interactions: Query<&Interaction>
) {
    let mouse_unused = p_interactions.iter().all(|&interaction| interaction == Interaction::None);
    p_action_sources.mouse_buttons = mouse_unused;
    p_action_sources.mouse_wheel   = mouse_unused;
}
