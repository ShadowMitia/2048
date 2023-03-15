use bevy::prelude::*;

// Two separate tween structs
// for simpler management of what data is interpolated

#[derive(Component)]
pub struct TweenScale {
    pub duration: f32,
    pub to: Vec3,
    pub from: Vec3,
    pub completed: bool,
    // TODO: make really private or remove?
    pub _starttime: Option<f32>,
}

#[derive(Component)]
pub struct TweenTranslation {
    pub duration: f32,
    pub to: Vec3,
    pub from: Vec3,
    pub completed: bool,
    // TODO: make really private or remove?
    pub _starttime: Option<f32>,
}

#[derive(Component)]
pub struct Cell {
    pub coord: UVec2,
}

#[must_use]
pub fn tween_scale(duration: f32, from: Vec3, to: Vec3) -> TweenScale {
    TweenScale {
        duration,
        to,
        from,
        completed: false,
        _starttime: None,
    }
}

#[must_use]
pub fn tween_translation(duration: f32, from: Vec3, to: Vec3) -> TweenTranslation {
    TweenTranslation {
        duration,
        to,
        from,
        completed: false,
        _starttime: None,
    }
}
