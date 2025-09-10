use crate::mission::*;
use crate::prelude::*;

pub fn spend_ticks(state: &mut MissionState, id: CharacterId, amount: i32) {
    state.level.find_character_mut(id).ticks -= amount;

    if let Some(next) = find_next_actor(&mut state.level) {
        if state.level.find_character(next).is_player() {
            state.current_actor = CurrentActor::PlayerStandardAction;
        } else {
            state.current_actor = CurrentActor::EnemyAction(next);
        }
    }
}

fn find_next_actor(level: &mut LevelState) -> Option<CharacterId> {
    // Sort by ticks with id as tiebreaker
    level.characters.sort_by_key(|c| (c.ticks, c.id));
    if let Some(actor) = level.characters.last() {
        let id = actor.id;
        if actor.ticks < TICKS_TO_ACT {
            let missing = TICKS_TO_ACT - actor.ticks;
            add_ticks(level, missing);
        }
        Some(id)
    } else {
        None
    }
}

fn add_ticks(level: &mut LevelState, amount: i32) {
    let mut effects_to_apply = vec![];

    for character in &mut level.characters {
        character.ticks += amount;

        for status in &mut character.status_effects {
            status.tick(amount);
        }

        let mut completed_status: Vec<_> = character
            .status_effects
            .extract_if(.., |s| s.duration <= 0)
            .collect();

        for completed in completed_status.drain(..) {
            if let Some(on_complete) = &completed.on_complete {
                if on_complete.reapply_count > 0 {
                    let mut reapply = completed.clone();
                    reapply.on_complete.as_mut().unwrap().reapply_count -= 1;
                    character.status_effects.push(reapply);
                }
                if let Some(complete_effect) = &on_complete.complete_effect {
                    effects_to_apply.push((character.id, complete_effect.clone(), completed.name));
                }
            }
        }
    }

    for (target, effect, effect_name) in effects_to_apply {
        apply_effect(
            level,
            EffectSource::StatusEffect(effect_name),
            target,
            &effect,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::engine::time::add_ticks;
    use crate::mission::*;

    #[test]
    fn reapply_status() {
        let (id, mut level) = create_test_map();

        let bat = level.find_character_mut(id);
        bat.status_effects.push(StatusEffect {
            name: "Burn".to_string(),
            kind: StatusEffectKind::RepeatingNegative,
            duration: 100,
            on_complete: Some(StatusEffectCompleteEffect {
                reapply_count: 3,
                complete_effect: Some(Box::new(Effect::ApplyDamage { damage: 1 })),
            }),
        });

        for i in 0..3 {
            add_ticks(&mut level, 100);
            let bat = level.find_character(id);
            assert!(bat.has_status_effect(StatusEffectKind::RepeatingNegative));
            assert_eq!(bat.health.max - i - 1, bat.health.current);
        }

        add_ticks(&mut level, 100);
        let bat = level.find_character(id);
        assert!(
            !level
                .find_character(id)
                .has_status_effect(StatusEffectKind::RepeatingNegative)
        );
        assert_eq!(bat.health.max - 4, bat.health.current);
    }
}
