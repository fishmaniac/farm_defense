use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use crate::game_manager::GameManager;
use crate::button_manager::ButtonManager;
use crate::button_manager;
use crate::tower_manager::TowerManager;

pub struct EventManager {
    event_pump: EventPump,
}

impl EventManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventManager {
        let event_pump = sdl_context.event_pump().unwrap(); 
        let event = EventManager {  
            event_pump,
        };
        event
    }

    pub fn do_event(
        &mut self, 
        game: &mut GameManager, 
        seed_buttons: &mut ButtonManager, 
        build_buttons: &mut ButtonManager,
        towers: &mut TowerManager,
    ) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    game.quit = true;
                    break
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.do_key_down(game, towers, keycode);
                    break
                }, 
                Event::KeyUp {keycode: Some(keycode), .. } => {
                    self.do_key_up(game, keycode);
                    break
                },
                Event::MouseMotion { x, y, .. } => {
                    game.mouse_point.x = x;
                    game.mouse_point.y = y;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    game.mouse_button = mouse_btn;
                }
                Event::MouseButtonUp { .. } => {
                    game.mouse_button = MouseButton::Unknown;
                },
                _ => {}
            }
        }
    }

    fn do_key_down(&mut self, 
        game: &mut GameManager, 
        towers: &mut TowerManager,
        keycode: sdl2::keyboard::Keycode,
    ) {
        match keycode {
            sdl2::keyboard::Keycode::P => { 
                towers.tower_vec.clear();
                game.target_vec.clear();
            },
            sdl2::keyboard::Keycode::Escape => game.quit = true,
            sdl2::keyboard::Keycode::Q => game.quit = true,
            sdl2::keyboard::Keycode::W => game.up = true,
            sdl2::keyboard::Keycode::S => game.down = true,
            sdl2::keyboard::Keycode::A => game.left = true,
            sdl2::keyboard::Keycode::D => game.right = true,
            sdl2::keyboard::Keycode::T => {
                if game.build_mode {
                    game.build_mode = false;
                }
                else {
                    game.build_mode = true;
                    game.seed_mode = false;
                    println!("BUILD MODE: {}", game.build_mode);
                    return
                }
                println!("BUILD MODE: {}", game.build_mode);
            }
            sdl2::keyboard::Keycode::Y => {
                if game.seed_mode {
                    game.seed_mode = false;
                }
                else {
                    game.seed_mode = true;
                    game.build_mode = false;
                    println!("SEED MODE: {}", game.seed_mode);
                    return
                }
                println!("SEED MODE: {}", game.seed_mode);
            }
            sdl2::keyboard::Keycode::Num1 => {
                println!("1 pressed\tcurrent seed: {}", game.current_seed);
            },
            sdl2::keyboard::Keycode::Num2 => {
                println!("2 pressed\tcurrent seed: {}", game.current_seed);
            },
            _ => println!("INVALID INPUT"),
        }
    }

    fn do_key_up(&mut self, 
        game: &mut GameManager, 
        keycode: sdl2::keyboard::Keycode
    ) {
        match keycode {
            sdl2::keyboard::Keycode::W => game.up = false,
            sdl2::keyboard::Keycode::S => game.down = false,
            sdl2::keyboard::Keycode::A => game.left = false,
            sdl2::keyboard::Keycode::D => game.right = false,
            _ => {}
        }
    }
}
