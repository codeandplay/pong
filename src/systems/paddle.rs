use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;

                let y_movement = (paddle_y + scaled_amount)
                    // use the minimum of this value and (paddle_y + scaled_amount)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                    // use the maximum of this value and (paddle_y + scaled_amount)
                    .max(PADDLE_HEIGHT * 0.5);

                transform.set_translation_y(y_movement);
            }
        }
    }
}
