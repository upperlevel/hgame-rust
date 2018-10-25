use std::vec::Vec;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub rigid_body: bool,
    pub no_clip: bool,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            x: 0.0,
            y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rigid_body: true,
            no_clip: false,
        }
    }
}

pub struct Level {
    width: f32,
    height: f32,
    ground_height: f32,
    gravity: f32,
    entities: Vec<Entity>,
}


impl Level {
    pub fn spawn(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn update(&mut self, delta: f32) {
        for entity in &mut self.entities {
            if entity.rigid_body {
                entity.x += entity.velocity_x * delta;
                entity.y += entity.velocity_y * delta;

                if !entity.no_clip && entity.y < self.ground_height {
                    entity.y = self.ground_height;
                }

                entity.velocity_y += self.gravity * delta;
            }
        }
    }

    pub fn new() -> Level {
        Level {
            width: 100.0,
            height: 100.0,
            ground_height: 10.0,
            gravity: 9.8,
            entities: Vec::new(),
        }
    }
}
