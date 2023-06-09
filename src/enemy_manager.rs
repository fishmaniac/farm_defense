use crate::constants;
use crate::event_manager;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::projectile_manager;
use crate::gui_manager;
use crate::tower_manager;

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    position: (usize, usize),
    priority: usize,
}

// Implement Ord trait for State to define the ordering in the BinaryHeap
impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the ordering to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

// Implement PartialOrd trait for State to enable comparison
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Enemy {
    pub elapsed_time: f64,
    pub cost_total: f32,
    pub final_path: Option<Vec<(usize, usize)>>,
    pub grid_index: (usize, usize),
    pub max_health: u16,
    pub health: u16,
    pub movement_speed: u8,
    pub attack_damage: u8,
    pub attack_radius: u8,
    pub attack_speed: u8,
    pub found_target: bool,
    pub direction: player_manager::Direction,
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct EnemyManager {
    pub enemy_vec: Vec<Enemy>,
}

impl EnemyManager {
    pub fn new () -> EnemyManager {
        let enemies = EnemyManager {
            enemy_vec: Vec::new(),
        };
        enemies
    }

    pub fn place_enemy(
        &mut self, 
        temp_tile: &level_manager::LevelTile, 
        index: (usize, usize),
    ) {
        match temp_tile.tile_data {
            TileData::Goblin => {
                let temp_enemy = self::Enemy {
                    elapsed_time: 0.0,
                    cost_total: 0.0,
                    final_path: None,
                    movement_speed: constants::ENEMY_GOBLIN_SPEED,
                    attack_damage: constants::ENEMY_GOBLIN_DAMAGE,
                    attack_radius: constants::ENEMY_GOBLIN_RADIUS,
                    attack_speed: constants::ENEMY_GOBLIN_ATTACK_SPEED,
                    max_health: constants::ENEMY_GOBLIN_HEALTH,
                    health: constants::ENEMY_GOBLIN_HEALTH,
                    found_target: false,
                    grid_index: index,
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            },
            _=> {
                let temp_enemy = self::Enemy {
                    elapsed_time: 0.0,
                    cost_total: 0.0,
                    final_path: None,
                    movement_speed: 1,
                    attack_damage: 1,
                    attack_radius: 1,
                    attack_speed: 1,
                    max_health: 1,
                    health: 1,
                    found_target: false,
                    grid_index: index,
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            }
        }
    }

    pub fn render_enemies(
        &mut self,
        game: &mut game_manager::GameManager, 
        events: &mut event_manager::EventManager,
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        level: &mut level_manager::LevelManager, 
        gui_manager: &mut gui_manager::GUIManager,
    ) -> Result<(), String> {
        for enemy in &mut self.enemy_vec {
            let pixel_index: (i32, i32) = (enemy.grid_index.0 as i32 * constants::TILE_SIZE as i32, enemy.grid_index.1 as i32 * constants::TILE_SIZE as i32);

            enemy.rect.set_x(pixel_index.0 as i32 - game.cam_x);
            enemy.rect.set_y(pixel_index.1 as i32 - game.cam_y);

            let texture = tex_man.load(&enemy.texture_path)?;

            game.canvas.copy_ex(
                &texture,
                None,
                enemy.rect,
                0.0,
                None,
                false,
                false,
            )?;

            Self::move_enemies(events, game, level, enemy);
            if enemy.health < enemy.max_health {
                gui_manager.render_health_bar_enemy(game, enemy);
            }
        }
        Ok(())
    }

    fn move_enemies (
        events: &mut event_manager::EventManager,
        game: &mut game_manager::GameManager,
        level: &mut level_manager::LevelManager, 
        enemy: &mut Enemy,
    ) {
        let has_no_targets: bool = !game.target_vec.is_empty() && enemy.final_path.is_none() && !enemy.found_target;
        let enemy_tuple_index = (enemy.grid_index.0 as i32, enemy.grid_index.1 as i32);

        let movement_interval = 1.0 / enemy.movement_speed as f64;
        enemy.elapsed_time += events.delta_time;
        let can_move: bool = !enemy.found_target && enemy.elapsed_time > movement_interval;
        /*         println!("Elapsed: {}\tInterval: {}", enemy.elapsed_time, movement_interval); */

        if can_move {
            if let Some(mut path) = enemy.final_path.take() {
                if let Some((col, row)) = path.first() {
                    /*                     if level.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_type == constants::TILE_TYPE_GOBLIN { */
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_type = level.level_vec[enemy.grid_index.0][enemy.grid_index.1].original_type;
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_data = TileData::None;
                    /*                     } */
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].is_occupied = false;
                    enemy.grid_index.0 = *col;
                    enemy.grid_index.1 = *row;
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].is_occupied = true;
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_type = constants::TILE_TYPE_GOBLIN;
                    level.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_data = TileData::Goblin;

                    path.remove(0);
                    enemy.final_path = Some(path);
                    enemy.elapsed_time = 0.0;
                }
            }
        }
        if has_no_targets {
            let random_index = game.frame_time as usize % game.target_vec.len();
            let random_tile = &mut level.level_vec[game.target_vec[random_index].0][game.target_vec[random_index].1];

            if random_tile.is_occupied {
                let rand_direction = game.frame_time % 4;
                println!("RAND DIRECTION: {}", rand_direction);
                match rand_direction {
                    0 => {
                        game.target_vec[random_index].0 += 1;
                    },
                    1 => {
                        if game.target_vec[random_index].0 > 0 {
                            game.target_vec[random_index].0 -= 1;
                        }
                        else {
                            game.target_vec[random_index].0 += 1;
                        }
                    },
                    2 => {
                        game.target_vec[random_index].1 += 1;
                    },
                    3 => {
                        if game.target_vec[random_index].1 > 0 {
                            game.target_vec[random_index].1 -= 1;
                        }
                        else {
                            game.target_vec[random_index].1 += 1;
                        }
                    },
                    _ => {},
                }
            }
            let target = game.target_vec[random_index];
            let target_tuple_index = (target.0 as i32, target.1 as i32);

            if !game.is_pathfinding && !tower_manager::TowerManager::is_within_area(enemy_tuple_index, target_tuple_index, enemy.attack_radius as i32) {
                Self::astar(enemy, target, &level.level_vec);
                game.is_pathfinding = true;
            }
            else {
                //idk abt this
                game.is_pathfinding = true;
            }
        }
    }

    pub fn astar(enemy: &mut Enemy, target: (usize, usize), level_vec: &[Vec<LevelTile>]) {
        println!("EXECUTING A*"); 
        let initial_state = PathState {
            position: enemy.grid_index,
            priority: heuristic(enemy.grid_index, target),
        };

        let mut frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();
        let mut priorities: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        let mut came_from: std::collections::HashMap<(usize, usize), (usize, usize)> = std::collections::HashMap::new();

        priorities.insert(enemy.grid_index, initial_state.priority);

        while let Some(current_state) = frontier.pop() {
            let current = current_state.position;

            if current == target {
                let mut path = vec![current];
                let mut current = current;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                enemy.final_path = Some(path);
            }

            let neighbors = get_neighbors(current, level_vec);

            for next in neighbors {
                let new_cost = 1;
                let priority = new_cost + heuristic(next, target);

                if !priorities.contains_key(&next) || priority < priorities[&next] {
                    priorities.insert(next, priority);
                    frontier.push(PathState {
                        position: next,
                        priority,
                    });
                    came_from.insert(next, current);
                }
            }
        }

        fn heuristic(position: (usize, usize), goal: (usize, usize)) -> usize {
            let (x1, y1) = position;
            let (x2, y2) = goal;

            let dx = (x1 as isize - x2 as isize).abs() as usize;
            let dy = (y1 as isize - y2 as isize).abs() as usize;

            dx + dy
        }
        fn get_neighbors(position: (usize, usize), level_vec: &[Vec<LevelTile>]) -> Vec<(usize, usize)> {
            let (x, y) = position;
            let width = level_vec[0].len();
            let height = level_vec.len();
            let mut neighbors = Vec::with_capacity(8);
            let top_tile = &level_vec[x][y - 1];
            let bottom_tile = &level_vec[x][y + 1]; 
            let left_tile = &level_vec[x - 1][y];
            let right_tile = &level_vec[x + 1][y];
            let top_left_tile = &level_vec[x - 1][y - 1];
            let top_right_tile = &level_vec[x - 1][y + 1];
            let bottom_left_tile = &level_vec[x + 1][y - 1];
            let bottom_right_tile = &level_vec[x + 1][y + 1];

            let tile_types_to_avoid = [
                constants::TILE_TYPE_WALL,
            ];

            //Up
            if y > 0 && !tile_types_to_avoid.contains(&top_tile.tile_type) && !top_tile.is_occupied {
                neighbors.push((x, y - 1));
            }
            //Down
            if y < height - 1 && !tile_types_to_avoid.contains(&bottom_tile.tile_type) && !bottom_tile.is_occupied {
                neighbors.push((x, y + 1));
            }
            //Left
            if x > 0 && !tile_types_to_avoid.contains(&left_tile.tile_type) && !left_tile.is_occupied {
                neighbors.push((x - 1, y));
            }
            //Right
            if x < width - 1 && !tile_types_to_avoid.contains(&right_tile.tile_type) && !right_tile.is_occupied {
                neighbors.push((x + 1, y));
            }
            // Top-left
            if x > 0 && y > 0 && !tile_types_to_avoid.contains(&top_left_tile.tile_type) && !top_left_tile.is_occupied {
                neighbors.push((x - 1, y - 1));
            }
            // Top-right
            if x > 0 && y < height - 1 && !tile_types_to_avoid.contains(&top_right_tile.tile_type) && !top_right_tile.is_occupied {
                neighbors.push((x - 1, y + 1));
            }
            // Bottom-left
            if x < width - 1 && y > 0 && !tile_types_to_avoid.contains(&bottom_left_tile.tile_type) && !bottom_left_tile.is_occupied {
                neighbors.push((x + 1, y - 1));
            }
            // Bottom-right
            if x < width - 1 && y < height - 1 && !tile_types_to_avoid.contains(&bottom_right_tile.tile_type) && !bottom_right_tile.is_occupied {
                neighbors.push((x + 1, y + 1));
            }            
            neighbors
        }
    }
}
