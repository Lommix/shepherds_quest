use bevy::math::Vec3;

pub fn quad_formation(count: usize, padding : f32) -> Vec<Vec3> {

    // sqt
    let extend = (count as f32).sqrt() as usize;
    let mut positions = Vec::with_capacity(count);
    for i in 0..count {
        let x = (i % extend) as f32 * padding;
        let y = (i / extend) as f32 * padding;
        positions.push(Vec3::new(x, y, 0.));
    }
    positions
}
