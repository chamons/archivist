use crate::mission::Data;

pub fn get_enemy_set_for_difficulty(data: &Data, difficulty: u32) -> Vec<String> {
    let under = data.get_enemies_at_level(difficulty - 1);
    let base = data.get_enemies_at_level(difficulty);
    let advanced = data.get_enemies_at_level(difficulty + 1);

    let mut set = vec![];

    set.append(&mut under.clone());
    for _ in 0..4 {
        set.append(&mut base.clone());
    }
    set.append(&mut advanced.clone());

    set
}
