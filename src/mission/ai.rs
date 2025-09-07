use crate::mission::*;
use crate::prelude::*;

use pathfinding::prelude::bfs;

pub fn default_ai_action(level: &LevelState, id: CharacterId) -> HandleInputResponse {
    if can_see_player(level, id) {
        if let Some(action) = check_skill_usage(level, id) {
            action
        } else {
            chase_attack_player(level, id)
        }
    } else {
        wander_action(level, id)
    }
}

pub fn check_skill_usage(level: &LevelState, id: CharacterId) -> Option<HandleInputResponse> {
    let enemy = level.find_character(id);
    if enemy.skills.is_empty() {
        return None;
    }

    for skill in &enemy.skills {
        if skill.cost.can_pay(enemy) {
            match &skill.targeting {
                SkillTargeting::Caster => {
                    if wants_caster_effect(&skill.effect, enemy) {
                        return Some(HandleInputResponse::Action(Some(
                            RequestedAction::UseSkill {
                                source: id,
                                target: id,
                                skill_name: skill.name.clone(),
                            },
                        )));
                    }
                }
                SkillTargeting::Ranged { max_range, sprite } => {
                    if let Some((target_id, target_position)) =
                        find_ranged_target(enemy, &skill.effect, *max_range, level)
                    {
                        return Some(HandleInputResponse::ChangeActor(CurrentActor::Animation(
                            AnimationInfo::new(
                                enemy.position,
                                target_position,
                                level,
                                sprite.clone(),
                                RequestedAction::UseSkill {
                                    source: id,
                                    target: target_id,
                                    skill_name: skill.name.clone(),
                                },
                            ),
                        )));
                    }
                }
            }
        }
    }

    None
}

fn wants_caster_effect(effect: &Effect, enemy: &Character) -> bool {
    match effect {
        Effect::ApplyDamage { .. } | Effect::ApplyWeaponDamage => false,
        Effect::Heal { amount } => enemy.health.max - enemy.health.current >= *amount,
    }
}

fn find_ranged_target(
    enemy: &Character,
    effect: &Effect,
    max_range: u32,
    level: &LevelState,
) -> Option<(CharacterId, Point)> {
    match effect {
        Effect::ApplyDamage { .. } | Effect::ApplyWeaponDamage => {
            let player = level.get_player();
            if clear_line_between(level, enemy.position, player.position, max_range) {
                Some((player.id, player.position))
            } else {
                None
            }
        }
        Effect::Heal { amount } => {
            for character in &level.characters {
                if !character.is_player()
                    && clear_line_between(level, enemy.position, character.position, max_range)
                    && character.health.max - character.health.current >= *amount
                {
                    return Some((character.id, character.position));
                }
            }
            None
        }
    }
}

pub fn chase_attack_player(level: &LevelState, id: CharacterId) -> HandleInputResponse {
    let enemy = level.find_character(id);
    let player = level.get_player();

    let path = bfs(
        &enemy.position,
        |p| adjacent_squares(level, *p, PathCharacterOptions::AllowEmptyOrPlayer),
        |p| *p == player.position,
    );
    if let Some(path) = path {
        // First position on path is current
        let dest = path[1];
        HandleInputResponse::Action(Some(handle_move_bump(enemy, dest, level)))
    } else {
        wander_action(level, enemy.id)
    }
}

pub fn wander_action(level: &LevelState, id: CharacterId) -> HandleInputResponse {
    let enemy = level.find_character(id);
    let options = adjacent_squares(
        level,
        enemy.position,
        PathCharacterOptions::AllowEmptyOrPlayer,
    );
    let selection = options.choose(&mut ::rand::rng());
    match selection {
        Some(position) => HandleInputResponse::Action(Some(RequestedAction::Move(id, *position))),
        None => HandleInputResponse::Action(Some(RequestedAction::Wait(id))),
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::*;
    use crate::prelude::*;

    #[test]
    fn chases_player() {
        let (id, level) = create_test_map();

        let action = chase_attack_player(&level, id);
        assert_eq!(
            action,
            HandleInputResponse::Action(Some(RequestedAction::Move(id, Point::new(1, 4))))
        );
    }
}
