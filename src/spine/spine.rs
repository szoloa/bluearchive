use std::sync::Arc;

use anyhow::Result;
use macroquad::prelude::*;
use rusty_spine::{
    AnimationStateData, Atlas, SkeletonBinary, SkeletonJson, controller::SkeletonController,
};

pub struct SpineDemo {
    atlas_path: String,
    skeleton_path: SkeletonPath,
    animation: String,
}

pub enum SkeletonPath {
    Json(String),
    Binary(String),
}

impl SpineDemo {
    pub fn new(atlas: String, skel: SkeletonPath, ani: String) -> Self {
        Self {
            atlas_path: atlas,
            skeleton_path: skel,
            animation: ani,
        }
    }
}

#[derive(Debug)]
pub struct Spine {
    pub controller: SkeletonController,
}

impl Spine {
    pub fn load(info: SpineDemo) -> Self {
        let atlas = Arc::new(Atlas::new_from_file(info.atlas_path).unwrap());
        let skeleton_data = Arc::new(match info.skeleton_path {
            SkeletonPath::Binary(path) => {
                let skeleton_binary = SkeletonBinary::new(atlas);
                skeleton_binary
                    .read_skeleton_data_file(path)
                    .unwrap_or_else(|_| panic!("Read skeleton failed"))
            }
            SkeletonPath::Json(path) => {
                let skeleton_json = SkeletonJson::new(atlas);
                skeleton_json
                    .read_skeleton_data_file(path)
                    .unwrap_or_else(|_| panic!("Read skeleton failed"))
            }
        });
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        let animation_series: Vec<String> = skeleton_data
            .animations()
            .map(|a| {
                let s = a.name();
                s.to_string()
            })
            .collect();
        debug!("Founded animation {:?}", animation_series);
        let mut controller = SkeletonController::new(skeleton_data, animation_state_data);
        controller
            .animation_state
            .set_animation_by_name(0, &info.animation, true)
            .unwrap();
        controller
            .animation_state
            .set_animation_by_name(1, "Idle_01", true)
            .unwrap();
        Self {
            controller: controller,
        }
    }
    pub fn get_mesh(&mut self, texture: Texture2D, x: f32, y: f32) -> Vec<Mesh> {
        let mut meshs: Vec<Mesh> = Vec::new();
        let renderables = self.controller.renderables();
        for renderable in renderables {
            let mut vertices = Vec::new();
            for index in 0..renderable.vertices.len() {
                vertices.push(Vertex::new(
                    renderable.vertices[index][0] * 0.4 + x,
                    -renderable.vertices[index][1] * 0.4 + y,
                    0.0,
                    renderable.uvs[index][0],
                    renderable.uvs[index][1],
                    Color {
                        r: renderable.color.r,
                        g: renderable.color.g,
                        b: renderable.color.b,
                        a: renderable.color.a,
                    },
                ));
            }

            let indices: Vec<u16> = renderable.indices.iter().map(|&i| i as u16).collect();
            let mesh = Mesh {
                vertices: vertices,
                indices: indices.clone(),
                texture: Some(texture.clone()),
            };
            meshs.push(mesh);
        }
        meshs
    }
    pub fn set_animationn(&mut self, animation_name: &str, index: usize) -> Result<()> {
        self.controller
            .animation_state
            .set_animation_by_name(index, animation_name, true)?;
        Ok(())
    }
}
