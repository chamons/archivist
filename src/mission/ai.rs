use crate::mission::*;
use crate::prelude::*;

use pathfinding::prelude::bfs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnemyMemory {
    last_known_position: Option<Point>,
}

impl EnemyMemory {
    pub fn new() -> Self {
        Self {
            last_known_position: None,
        }
    }
}

pub fn default_ai_action(level: &mut LevelState, id: CharacterId) -> HandleInputResponse {
    if can_see_player(level, id) {
        remember_last_position(level, id);
        if let Some(action) = check_skill_usage(level, id) {
            action
        } else {
            chase_attack_player(level, id)
        }
    } else {
        if let Some(last_position) = remembered_last_position_to_head_to(level, id) {
            head_to_position_action(level, id, last_position)
        } else {
            wander_action(level, id)
        }
    }
}

fn remembered_last_position_to_head_to(level: &LevelState, id: CharacterId) -> Option<Point> {
    let enemy = level.find_character(id);

    if let Some(enemy_memory) = &enemy.enemy_memory {
        enemy_memory.last_known_position.clone()
    } else {
        None
    }
}

fn remember_last_position(level: &mut LevelState, id: CharacterId) {
    let player_position = level.get_player().position;
    let enemy = level.find_character_mut(id);

    if let Some(enemy_memory) = &mut enemy.enemy_memory {
        enemy_memory.last_known_position = Some(player_position);
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
        Effect::ApplyDamage { .. } => false,
        Effect::Heal { amount } => enemy.health.max - enemy.health.current >= *amount,
        Effect::AddStatus { effect } => effect.is_positive(),
    }
}

fn find_ranged_target(
    enemy: &Character,
    effect: &Effect,
    max_range: u32,
    level: &LevelState,
) -> Option<(CharacterId, Point)> {
    match effect {
        Effect::ApplyDamage { .. } => {
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
        Effect::AddStatus { effect } => {
            for character in &level.characters {
                if clear_line_between(level, enemy.position, character.position, max_range)
                    && ((character.is_player() && !effect.is_positive())
                        || !character.is_player() && effect.is_positive())
                {
                    return Some((character.id, character.position));
                }
            }
            None
        }
    }
}

pub fn chase_attack_player(level: &mut LevelState, id: CharacterId) -> HandleInputResponse {
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

pub fn head_to_position_action(
    level: &mut LevelState,
    id: CharacterId,
    position: Point,
) -> HandleInputResponse {
    let enemy_position = level.find_character(id).position;

    let path = bfs(
        &enemy_position,
        |p| adjacent_squares(level, *p, PathCharacterOptions::AllowEmptyOrPlayer),
        |p| *p == position,
    );

    if let Some(path) = path {
        // First position on path is current
        let dest = path[1];
        move_to(level, id, dest)
    } else {
        wander_action(level, id)
    }
}

pub fn wander_action(level: &mut LevelState, id: CharacterId) -> HandleInputResponse {
    let enemy = level.find_character(id);
    let options = adjacent_squares(
        level,
        enemy.position,
        PathCharacterOptions::AllowEmptyOrPlayer,
    );
    let selection = options.choose(&mut ::rand::rng());
    match selection {
        Some(position) => move_to(level, id, *position),
        None => HandleInputResponse::Action(Some(RequestedAction::Wait(id))),
    }
}

fn move_to(level: &mut LevelState, id: CharacterId, position: Point) -> HandleInputResponse {
    let enemy = level.find_character_mut(id);
    // If we move onto the last known position of the enemy, clear our memory
    if let Some(enemy_memory) = &mut enemy.enemy_memory {
        if enemy_memory.last_known_position == Some(position) {
            enemy_memory.last_known_position = None;
        }
    }
    HandleInputResponse::Action(Some(RequestedAction::Move(id, position)))
}

#[cfg(test)]
mod tests {
    use crate::mission::*;
    use crate::prelude::*;

    #[test]
    fn chases_player() {
        let (id, mut level) = create_test_map();

        let action = chase_attack_player(&mut level, id);
        assert_eq!(
            action,
            HandleInputResponse::Action(Some(RequestedAction::Move(id, Point::new(1, 4))))
        );
    }

    #[test]
    fn heads_to_last_position_of_player() {
        let (id, mut level) = create_test_map();

        // First head towards the player
        let action = default_ai_action(&mut level, id);
        assert_eq!(
            action,
            HandleInputResponse::Action(Some(RequestedAction::Move(id, Point::new(1, 4))))
        );
        level.find_character_mut(id).position = Point::new(1, 4);

        // Teleport the player away, far enough we can't be seen now
        level.get_player_mut().position = Point::new(40, 40);

        // We should keep heading to the players last position
        for i in 0..3 {
            let action = default_ai_action(&mut level, id);
            assert_eq!(
                action,
                HandleInputResponse::Action(Some(RequestedAction::Move(id, Point::new(1, 3 - i))))
            );
            level.find_character_mut(id).position = Point::new(1, 3 - i);
        }

        // And then have forgotten
        assert!(
            level
                .find_character(id)
                .enemy_memory
                .as_ref()
                .unwrap()
                .last_known_position
                .is_none()
        );
    }
}
