use rand::Rng;
use ray_tracer_rs::{camera, progress, scenes, vec3};

fn main() {
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    // let width = 384;
    let width = 512;
    let height = (width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 5000;
    let max_depth = 100;

    print!("P3\n{} {}\n255\n", width, height);

    let world = scenes::cornell_box::scene();

    // NOTE: The following code is for the camera position for random scenes
    // let lookfrom = vec3::Point3::new(13.0, 2.0, 3.0);
    // let lookat = vec3::Point3::new(0.0, 0.0, 0.0);
    // NOTE: The following code is for the camera position for the cornell box scene
    let lookfrom = vec3::Point3::new(278.0, 278.0, -800.0);
    let lookat = vec3::Point3::new(278.0, 278.0, 0.0);
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let background = vec3::Color::zero();

    let mut pb = progress::ProgressBar::new((width * height) as usize);
    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color = vec3::Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (width - 1) as f64;
                let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&background, &world, max_depth);
            }
            pb.update();
            pixel_color.write(samples_per_pixel);
        }
    }

    eprintln!("\n\nDone.\n"); // indicate completion
}
