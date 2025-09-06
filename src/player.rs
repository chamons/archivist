use crate::prelude::*;

pub fn get_player_action(
    player: &Character,
    level: &LevelState,
    screen: &mut Screen,
) -> HandleInputResponse {
    if let Some(movement_delta) = handle_movement_key() {
        HandleInputResponse::Action(Some(handle_move_bump(
            player,
            player.position + movement_delta,
            level,
        )))
    } else if is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::Kp5) {
        HandleInputResponse::Action(Some(RequestedAction::Wait(player.id)))
    } else if let Some(index) = skill_index_from_number_key_pressed() {
        if let Some(skill) = player.skills.get(index) {
            if player.will.has_enough(skill.cost) {
                screen.push_floating_text(&format!("Targeting {}", skill.name));
                match &skill.targeting {
                    SkillTargeting::Caster => {
                        HandleInputResponse::Action(Some(RequestedAction::UseEffect {
                            source: player.id,
                            target: player.id,
                            effect: skill.effect.clone(),
                            cost: skill.cost,
                        }))
                    }
                    SkillTargeting::Ranged(animation_sprite_kind) => {
                        HandleInputResponse::ChangeActor(CurrentActor::PlayerTargeting(
                            TargetingInfo::new(
                                player.position,
                                TargetEffect {
                                    effect: skill.effect.clone(),
                                    spite: animation_sprite_kind.clone(),
                                    cost: skill.cost,
                                },
                            ),
                        ))
                    }
                }
            } else {
                screen.push_floating_text(&format!("Not enough will to use {}", skill.name));
                HandleInputResponse::Action(None)
            }
        } else {
            HandleInputResponse::Action(None)
        }
    } else if is_key_pressed(KeyCode::F1) {
        #[cfg(debug_assertions)]
        {
            HandleInputResponse::Action(Some(RequestedAction::DebugMenu(DebugRequest::Save)))
        }
        #[cfg(not(debug_assertions))]
        {
            HandleInputResponse::Action(None)
        }
    } else if is_key_pressed(KeyCode::F2) {
        #[cfg(debug_assertions)]
        {
            HandleInputResponse::Action(Some(RequestedAction::DebugMenu(DebugRequest::Load)))
        }
        #[cfg(not(debug_assertions))]
        {
            HandleInputResponse::Action(None)
        }
    } else if is_key_pressed(KeyCode::F3) {
        #[cfg(debug_assertions)]
        {
            HandleInputResponse::Action(Some(RequestedAction::DebugMenu(DebugRequest::DumpState)))
        }
        #[cfg(not(debug_assertions))]
        {
            HandleInputResponse::Action(None)
        }
    } else {
        HandleInputResponse::Action(None)
    }
}

pub fn skill_index_from_number_key_pressed() -> Option<usize> {
    if is_key_pressed(KeyCode::Key0) {
        Some(9)
    } else if is_key_pressed(KeyCode::Key1) {
        Some(0)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(1)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(2)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(3)
    } else if is_key_pressed(KeyCode::Key5) {
        Some(4)
    } else if is_key_pressed(KeyCode::Key6) {
        Some(5)
    } else if is_key_pressed(KeyCode::Key7) {
        Some(6)
    } else if is_key_pressed(KeyCode::Key8) {
        Some(7)
    } else if is_key_pressed(KeyCode::Key9) {
        Some(8)
    } else {
        None
    }
}
