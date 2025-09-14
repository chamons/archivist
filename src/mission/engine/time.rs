use crate::mission::*;
use crate::prelude::*;

pub fn spend_ticks(state: &mut MissionState, id: CharacterId, amount: i32) {
    state.level.find_character_mut(id).ticks -= amount;

    if let Some(next) = find_next_actor(&mut state.level) {
        // There is a chance the next actor died when time moved forward
        if state.level.does_character_exist(next) {
            if state.level.find_character(next).is_player() {
                state.current_actor = CurrentActor::PlayerStandardAction;
            } else {
                state.current_actor = CurrentActor::EnemyAction(next);
            }
        } else {
            // Spend zero ticks to pick the next valid entry
            spend_ticks(state, id, 0);
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
        if !character.has_status_effect(StatusEffectKind::Stun) {
            character.ticks += amount;
        }

        for status in &mut character.status_effects {
            status.tick(amount);
        }

        for skill in &mut character.skills {
            match &mut skill.cost {
                SkillCost::Cooldown { ticks, .. } => {
                    if *ticks > 0 {
                        *ticks -= amount;
                        *ticks = *ticks.max(&mut 0);
                    }
                }
                _ => {}
            }
        }

        let mut completed_status: Vec<_> = character
            .status_effects
            .extract_if(.., |s| match s.duration {
                Some(duration) => duration <= 0,
                None => false,
            })
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
            &EffectSource::StatusEffect(effect_name),
            target,
            &effect,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::engine::time::add_ticks;
    use crate::mission::*;
    use crate::util::Point;

    #[test]
    fn reapply_status() {
        let (id, mut level) = create_test_map();

        let bat = level.find_character_mut(id);
        bat.status_effects.push(StatusEffect {
            name: "Burn".to_string(),
            kind: StatusEffectKind::RepeatingNegative,
            duration: Some(100),
            on_complete: Some(StatusEffectCompleteEffect {
                reapply_count: 3,
                complete_effect: Some(Box::new(Effect::ApplyDamage {
                    damage: 2,
                    on_hit: None,
                    on_hit_self: None,
                    pierce: DamagePierce::None,
                })),
            }),
        });

        let mut last_health = bat.health.current;

        for _ in 0..3 {
            add_ticks(&mut level, 100);
            let bat = level.find_character(id);
            assert!(bat.has_status_effect(StatusEffectKind::RepeatingNegative));
            assert!(bat.health.current < last_health);
            last_health = bat.health.current;
        }

        add_ticks(&mut level, 100);
        let bat = level.find_character(id);
        assert!(
            !level
                .find_character(id)
                .has_status_effect(StatusEffectKind::RepeatingNegative)
        );
        assert!(bat.health.current < last_health);
    }

    #[test]
    fn immortal_status() {
        let (id, mut level) = create_test_map();

        let bat = level.find_character_mut(id);
        bat.status_effects.push(StatusEffect {
            name: "Lifesteal".to_string(),
            kind: StatusEffectKind::Lifesteal,
            duration: None,
            on_complete: None,
        });

        for _ in 0..100 {
            add_ticks(&mut level, 100);
            let bat = level.find_character(id);
            assert!(bat.has_status_effect(StatusEffectKind::Lifesteal));
        }
    }

    #[test]
    fn stun_prevents_tick_gain_for_period() {
        let (id, mut level) = create_test_map();

        let bat = level.find_character_mut(id);
        bat.status_effects.push(StatusEffect {
            name: "Stunned".to_string(),
            kind: StatusEffectKind::Stun,
            duration: Some(200),
            on_complete: None,
        });

        add_ticks(&mut level, 100);
        assert_eq!(level.find_character(id).ticks, 0);
        assert!(
            level
                .find_character(id)
                .has_status_effect(StatusEffectKind::Stun)
        );
        add_ticks(&mut level, 100);
        assert_eq!(level.find_character(id).ticks, 0);
        assert!(
            !level
                .find_character(id)
                .has_status_effect(StatusEffectKind::Stun)
        );
        add_ticks(&mut level, 100);
        assert_eq!(level.find_character(id).ticks, 100);
    }

    #[test]
    fn skills_recharge() {
        let (id, mut level) = create_test_map();

        let bat = level.find_character_mut(id);
        bat.skills.push(Skill {
            name: "Bite".to_string(),
            cost: SkillCost::Cooldown {
                ticks: 200,
                cost: 200,
            },
            effect: Effect::ApplyDamage {
                damage: 2,
                on_hit: None,
                on_hit_self: None,
                pierce: DamagePierce::None,
            },
            targeting: SkillTargeting::Ranged {
                max_range: 1,
                sprite: AnimationSpriteKind::SingleFrame(Point::zero()),
            },
        });

        add_ticks(&mut level, 100);
        assert_eq!(
            level.find_character(id).skills[0].cost,
            SkillCost::Cooldown {
                ticks: 100,
                cost: 200
            }
        );

        add_ticks(&mut level, 150);
        assert_eq!(
            level.find_character(id).skills[0].cost,
            SkillCost::Cooldown {
                ticks: 0,
                cost: 200
            }
        );
    }
}
