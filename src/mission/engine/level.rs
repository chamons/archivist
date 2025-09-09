use macroquad::input::mouse_position;
use macroquad::shapes::draw_rectangle_lines;
use macroquad::{shapes::draw_rectangle, text::draw_text, window::screen_width};

use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelState {
    pub map: Map,
    pub characters: Vec<Character>,
    pub items: Vec<(Point, Item)>,
    visibility: VisibilityMap,
}

impl LevelState {
    pub fn new(map: Map, characters: Vec<Character>, items: Vec<(Point, Item)>) -> Self {
        let mut this = Self {
            map,
            characters,
            items,
            visibility: VisibilityMap::new(),
        };
        this.update_visibility();
        this
    }

    pub fn get_player(&self) -> &Character {
        self.characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    pub fn get_player_mut(&mut self) -> &mut Character {
        self.characters
            .iter_mut()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    pub fn find_character(&self, id: CharacterId) -> &Character {
        self.characters
            .iter()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_mut(&mut self, id: CharacterId) -> &mut Character {
        self.characters
            .iter_mut()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_at_position(&self, position: Point) -> Option<&Character> {
        self.characters.iter().find(|c| c.position == position)
    }

    pub fn character_can_enter(&self, point: Point) -> bool {
        self.map.in_bounds(point) && self.map.get(point).can_enter()
    }

    pub fn remove_character(&mut self, id: CharacterId) {
        self.characters.retain(|c| c.id != id);
    }

    pub fn render(&self, screen: &mut Screen) {
        self.map.render(screen, &self.visibility);

        for (item_position, item) in &self.items {
            if screen.camera.is_in_view(*item_position) && self.visibility.get(*item_position) {
                item.render(screen, *item_position);
            }
        }

        for character in &self.characters {
            if screen.camera.is_in_view(character.position)
                && self.visibility.get(character.position)
            {
                character.render(screen);
            }
        }

        self.render_hud(screen);
        screen.render_floating_text();
    }

    fn render_hud(&self, screen: &Screen) {
        let player = self.get_player();

        let health = player.health.clone();
        let health_percentage = health.percentage();

        const BAR_PADDING_X: f32 = 4.0;
        const BAR_PADDING_Y: f32 = 2.0;
        draw_rectangle(0.0, 0.0, screen_width(), 18.0, BLACK);
        draw_rectangle(
            BAR_PADDING_X,
            BAR_PADDING_Y,
            (screen_width() - BAR_PADDING_X * 2.0) * health_percentage,
            16.0,
            color_for_health(health_percentage),
        );

        Screen::draw_centered_text(
            &format!("{}/{}", health.current, health.max),
            17,
            15.0,
            None,
        );

        let will = player.will.clone();
        let will_percentage = will.percentage();

        draw_rectangle(
            BAR_PADDING_X,
            BAR_PADDING_Y + 16.0,
            (screen_width() - BAR_PADDING_X * 2.0) * will_percentage,
            16.0,
            color_for_will(will_percentage),
        );

        Screen::draw_centered_text(&format!("{}/{}", will.current, will.max), 17, 31.0, None);

        let offset = self.draw_skills();
        self.draw_items(offset);

        let mouse_position = mouse_position();
        let position = Point::new(
            (mouse_position.0 as i32 / 24) + screen.camera.left_x,
            (mouse_position.1 as i32 / 24) + screen.camera.top_y,
        );

        if let Some(moused_over) = self.find_character_at_position(position) {
            let y = if mouse_position.1 < 200.0 {
                mouse_position.1 + 55.0
            } else {
                mouse_position.1 - 80.0
            };
            draw_rectangle(mouse_position.0 - 20.0, y - 30.0, 200.0, 100.0, BLACK);
            draw_rectangle_lines(mouse_position.0 - 20.0, y - 30.0, 200.0, 100.0, 3.0, WHITE);
            draw_text(
                &format!("Name: {}", moused_over.name),
                mouse_position.0,
                y - 5.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Health: {}", moused_over.health.max),
                mouse_position.0,
                y + 15.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Will: {}", moused_over.will.max),
                mouse_position.0,
                y + 35.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Damage: {}", moused_over.weapon.damage),
                mouse_position.0,
                y + 55.0,
                20.0,
                WHITE,
            );
        }
        if let Some(item) = self.items.iter().find(|(p, _)| *p == position) {
            let y = if mouse_position.1 < 100.0 {
                mouse_position.1 + 55.0
            } else {
                mouse_position.1 - 15.0
            };
            draw_rectangle(mouse_position.0 - 20.0, y - 30.0, 200.0, 40.0, BLACK);
            draw_rectangle_lines(mouse_position.0 - 20.0, y - 30.0, 200.0, 40.0, 3.0, WHITE);
            draw_text(
                &format!("{}", item.1.name),
                mouse_position.0,
                y - 5.0,
                20.0,
                WHITE,
            );
        }
        if self.map.in_bounds(position) {
            match self.map.get(position).kind {
                TileKind::Exit => {
                    let y = if mouse_position.1 < 100.0 {
                        mouse_position.1 + 55.0
                    } else {
                        mouse_position.1 - 15.0
                    };
                    draw_rectangle(mouse_position.0 - 20.0, y - 30.0, 200.0, 40.0, BLACK);
                    draw_rectangle_lines(
                        mouse_position.0 - 20.0,
                        y - 30.0,
                        200.0,
                        40.0,
                        3.0,
                        WHITE,
                    );
                    draw_text(
                        &format!("Stairs Up"),
                        mouse_position.0,
                        y - 5.0,
                        20.0,
                        WHITE,
                    );
                }
                _ => {}
            }
        }
    }

    fn draw_skills(&self) -> f32 {
        let player = self.get_player();
        let mut offset = 60.0;

        for (i, skill) in player.skills.iter().enumerate() {
            offset += 18.0;

            let cost = match &skill.cost {
                SkillCost::Will(cost) => cost.to_string(),
                SkillCost::Charges { remaining, total } => format!("{remaining}/{total}"),
            };
            let color = if skill.cost.can_pay(player) {
                WHITE
            } else {
                RED
            };
            draw_text(
                &format!("{} - {} ({cost})", Self::skill_index_to_key(i), skill.name),
                screen_width() - 250.0,
                offset,
                22.0,
                color,
            );
        }
        offset + 35.0
    }

    fn draw_items(&self, mut offset: f32) {
        let player = self.get_player();

        if !player.carried_items.is_empty() {
            draw_text("Inventory:", screen_width() - 250.0, offset, 22.0, WHITE);
            offset += 22.0;
        }

        for (i, item) in player.carried_items.iter().enumerate() {
            draw_text(
                &item.name,
                screen_width() - 230.0,
                offset + 18.0 * i as f32,
                22.0,
                WHITE,
            );
        }
    }

    fn skill_index_to_key(index: usize) -> usize {
        match index {
            0..=8 => index + 1,
            9 => 0,
            _ => panic!("Unable to map index with more than 10 skills?"),
        }
    }

    pub fn update_visibility(&mut self) {
        self.visibility = self.map.compute_visibility(self.get_player().position);
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let position = Point::new(x, y);
                if self.visibility.get(position) {
                    self.map.set_known(position);
                }
            }
        }
    }
}
