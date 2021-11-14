use sdl2::event::Event;

fn event_handler(player: &mut Player) {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'running;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Left;
            player.direction2.push_back(Direction::Left);
        },
        Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Right;
            player.direction2.push_back(Direction::Right);
        },
        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Up;
            player.direction2.push_back(Direction::Up);
        },
        Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Down;
            player.direction2.push_back(Direction::Down);
        },
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            player.direction2.pop_front();
            player.speed = 0;
        },
        _ => {}
    }
}