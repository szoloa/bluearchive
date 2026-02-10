use bluearchive::spine::*;
use macroquad::prelude::*;

#[macroquad::main("Character")]
async fn main() {
    let character_path = "arona";
    let path = format!(
        "/home/kina/temp/code/rust/spine/data/{}/{}",
        character_path, character_path
    );
    let texture_path = format!("{}_spr.png", path);

    let texture = load_texture(&texture_path)
        .await
        .unwrap_or_else(|e| panic!("Can not load character texture. {:?}", e));
    let atlas_path = format!("{}_spr.atlas", path);
    let skel_path = format!("{}_spr.skel", path);

    let spine_demo = SpineDemo::new(
        atlas_path,
        SkeletonPath::Binary(skel_path),
        "00".to_string(),
    );

    let mut last_frame = get_time();
    let mut spine = Spine::load(spine_demo);
    loop {
        let current_time = get_time();
        let delta_time = (current_time - last_frame) as f32;
        last_frame = current_time;
        spine.controller.update(delta_time);

        let meshs = spine.get_mesh(texture.clone(), screen_width() / 2., screen_height());
        for i in &meshs {
            draw_mesh(i);
        }
        next_frame().await
    }
}
