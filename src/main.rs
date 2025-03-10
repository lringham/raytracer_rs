mod geometry;
mod ppm;
mod raycast;
mod vec3f;

use raycast::Raycastable;
use vec3f::Vec3f;

fn color_to_tuple(color: Vec3f) -> (u8, u8, u8) {
    (
        (255.0 * color.x.sqrt()) as u8,
        (255.0 * color.y.sqrt()) as u8,
        (255.0 * color.z.sqrt()) as u8,
    )
}

fn main() {
    // Create scene
    // TODO: load from json using `serde`
    let camera_position = Vec3f::new(0.0, 0.0, 1.0);
    let sphere = geometry::Sphere::new(Vec3f::new(0.0, 0.0, -2.0), 1.75);
    let light_pos = Vec3f::new(0.0, 5.0, 2.0);
    let light_col = Vec3f::new(0.392157, 0.584314, 0.929412);
    let bg_color = Vec3f::new(0.1, 0.1, 0.1);

    // Render
    let px_size = 0.01;
    let width = 500;
    let height = 500;
    let mut framebuffer = ppm::Ppm::new(width, height);
    for y in 0..width {
        for x in 0..height {
            let dir = Vec3f::new(
                px_size * (x - width / 2) as f32,
                px_size * (y - height / 2) as f32,
                -1.0,
            )
            .normalized();
            let ray = geometry::Ray::new(camera_position, dir);
            let hit = sphere.raycast(&ray);
            if let Some(res) = hit {
                let l = (light_pos - res.hit).normalized();

                // ambient
                let ambient = 0.1;

                // diffuse
                let lambertian = res.normal.dot(&l);
                let lambertian = if 0.0 > lambertian { 0.0 } else { lambertian };

                // specular

                //
                let color = light_col * (ambient + lambertian);

                framebuffer.set(x, height - 1 - y, color_to_tuple(color));
            } else {
                framebuffer.set(x, height - 1 - y, color_to_tuple(bg_color));
            }
        }
    }

    if let Err(e) = ppm::save(&framebuffer, String::from("image.ppm")) {
        eprintln!("Error saving image: {}", e);
    }
}
