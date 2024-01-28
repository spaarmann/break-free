use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        Anchor, MaterialMesh2dBundle,
    },
    window::WindowResolution,
};

const COLOR_A: Color = Color::rgb(0.41961, 1.0, 0.98431);
const NAME_A: &'static str = "Blue";
const COLOR_B: Color = Color::rgb(1.0, 0.62353, 0.41961);
const NAME_B: &'static str = "Red";

const BACKGROUND_COLOR: Color = Color::rgb(0.43922, 0.50196, 0.49804);

const TILE_COUNT: i64 = 20;
const TILE_SIZE: f32 = 25.0;
const BALL_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCOREBOARD_PADDING: f32 = 10.0;

const BALL_INITIAL_DIR_A: Vec2 = Vec2::new(1.0, 0.4);
const BALL_INITIAL_DIR_B: Vec2 = Vec2::new(-1.0, -0.3);
const BALL_SPEED: f32 = 800.0;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Team {
    A,
    B,
}

#[derive(Resource)]
struct Scoreboard {
    tiles_a: i64,
    tiles_b: i64,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct IsTeam(Team);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    team: IsTeam,
    vel: Velocity,
    mat_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#break-free-canvas".into()),
                resolution: WindowResolution::new(700.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_velocity, handle_collisions).chain())
        .add_systems(
            Update,
            (
                update_tile_colors,
                update_scoreboard,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

impl Team {
    fn name(self) -> &'static str {
        match self {
            Team::A => NAME_A,
            Team::B => NAME_B,
        }
    }

    fn color(self) -> Color {
        match self {
            Team::A => COLOR_A,
            Team::B => COLOR_B,
        }
    }

    fn opposite(self) -> Team {
        match self {
            Team::A => Team::B,
            Team::B => Team::A,
        }
    }
}

impl Scoreboard {
    fn score_for_team(&mut self, team: Team) -> &mut i64 {
        match team {
            Team::A => &mut self.tiles_a,
            Team::B => &mut self.tiles_b,
        }
    }
    fn flip(&mut self, prev: Team, new: Team) {
        *self.score_for_team(prev) -= 1;
        *self.score_for_team(new) += 1;
    }
}

impl BallBundle {
    fn new(
        team: Team,
        pos: Vec3,
        vel: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            ball: Ball,
            team: IsTeam(team),
            vel: Velocity(vel),
            mat_mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(team.opposite().color())),
                transform: Transform::from_translation(pos).with_scale(BALL_SIZE),
                ..default()
            },
        }
    }
}

fn spawn_wall(commands: &mut Commands, pos: Vec3, scale: Vec3) {
    commands.spawn((
        Collider,
        Transform {
            translation: pos,
            scale,
            ..default()
        }, //SpriteBundle {
           //    transform: Transform {
           //        translation: pos,
           //        scale,
           //        ..default()
           //    },
           //    sprite: Sprite {
           //        color: Color::rgb(0.0, 1.0, 0.0),
           //        ..default()
           //    },
           //    ..default()
           //},
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BallBundle::new(
        Team::A,
        Vec3::new(-50.0, 0.0, 1.0),
        BALL_INITIAL_DIR_A.normalize() * BALL_SPEED,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(BallBundle::new(
        Team::B,
        Vec3::new(50.0, 0.0, 1.0),
        BALL_INITIAL_DIR_B.normalize() * BALL_SPEED,
        &mut meshes,
        &mut materials,
    ));

    let tiles_start = Vec2::new(
        (-TILE_COUNT / 2) as f32 * TILE_SIZE,
        (-TILE_COUNT / 2) as f32 * TILE_SIZE,
    );
    let tiles_end = Vec2::new(
        (TILE_COUNT / 2) as f32 * TILE_SIZE,
        (TILE_COUNT / 2) as f32 * TILE_SIZE,
    );
    for x in 0..TILE_COUNT {
        for y in 0..TILE_COUNT {
            let pos =
                tiles_start + Vec2::new((x as f32 + 0.5) * TILE_SIZE, (y as f32 + 0.5) * TILE_SIZE);
            let team = if x < TILE_COUNT / 2 { Team::A } else { Team::B };

            commands.spawn((
                Tile,
                IsTeam(team),
                Collider,
                SpriteBundle {
                    transform: Transform::from_translation(pos.extend(0.0))
                        .with_scale(Vec2::splat(TILE_SIZE).extend(0.0)),
                    sprite: Sprite {
                        color: team.color(),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }

    spawn_wall(
        &mut commands,
        Vec3::new(tiles_start.x - TILE_SIZE / 2.0, 0.0, 0.0),
        Vec3::new(TILE_SIZE, TILE_SIZE * TILE_COUNT as f32, 0.0),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(tiles_end.x + TILE_SIZE / 2.0, 0.0, 0.0),
        Vec3::new(TILE_SIZE, TILE_SIZE * TILE_COUNT as f32, 0.0),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, tiles_start.y - TILE_SIZE / 2.0, 0.0),
        Vec3::new(TILE_SIZE * TILE_COUNT as f32, TILE_SIZE, 0.0),
    );
    spawn_wall(
        &mut commands,
        Vec3::new(0.0, tiles_end.y + TILE_SIZE / 2.0, 0.0),
        Vec3::new(TILE_SIZE * TILE_COUNT as f32, TILE_SIZE, 0.0),
    );

    let init_count = TILE_COUNT * TILE_COUNT / 2;
    let style = TextStyle {
        font_size: SCOREBOARD_FONT_SIZE,
        color: SCOREBOARD_COLOR,
        ..default()
    };

    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            //TextSection::new(format!("{}: ", Team::A.name()), style.clone()),
            TextSection::new("", style.clone()),
            TextSection::new(init_count.to_string(), style.clone()),
            //TextSection::new(format!(", {}: ", Team::B.name()), style.clone()),
            TextSection::new(" | ", style.clone()),
            TextSection::new(init_count.to_string(), style),
        ]),
        text_anchor: Anchor::TopCenter,
        transform: Transform::from_translation(Vec3::new(
            0.0,
            tiles_start.y - SCOREBOARD_PADDING,
            0.0,
        )),
        ..default()
    });

    commands.insert_resource(Scoreboard {
        tiles_a: init_count,
        tiles_b: init_count,
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
fn handle_collisions(
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform, &IsTeam), With<Ball>>,
    mut collider_query: Query<(&Transform, Option<&mut IsTeam>), (With<Collider>, Without<Ball>)>,
) {
    for (mut ball_velocity, ball_transform, ball_team) in &mut ball_query {
        let ball_size = ball_transform.scale.truncate();

        for (transform, maybe_team) in &mut collider_query {
            if let Some(ref team) = maybe_team {
                if team.0 == ball_team.0 {
                    // Balls don't collider with tiles of their own teams.
                    continue;
                }
            }

            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                if let Some(mut team) = maybe_team {
                    // Flip tile, adjust scoreboard.
                    scoreboard.flip(team.0, ball_team.0);
                    team.as_mut().0 = ball_team.0;
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

fn update_tile_colors(mut query: Query<(&IsTeam, &mut Sprite), With<Tile>>) {
    for (team, mut sprite) in &mut query {
        sprite.color = team.0.color();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.tiles_a.to_string();
    text.sections[3].value = scoreboard.tiles_b.to_string();
}
