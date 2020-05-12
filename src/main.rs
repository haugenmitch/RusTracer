fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let r = (i as f64) / (image_height as f64);
            let g = (j as f64) / (image_width as f64);
            let b = 0.25 as f64;

            let ir = (r * 255.0).round();
            let ig = (g * 255.0).round();
            let ib = (b * 255.0).round();

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
