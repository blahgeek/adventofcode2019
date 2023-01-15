
fn solve_a(content: &str, width: usize, height: usize) -> usize {
    let bytes_per_image = width * height;
    let mut layers: Vec<&str> = (0_usize..(content.len() / bytes_per_image))
        .map(|i| &content[i*bytes_per_image..(i+1)*bytes_per_image])
        .collect();
    layers.sort_by_key(|layer| layer.chars().filter(|v| *v == '0').count());
    layers[0].chars().filter(|v| *v == '1').count()
        * layers[0].chars().filter(|v| *v == '2').count()
}

fn solve_b(content: &str, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let res = content.chars()
                .skip(y*width+x).step_by(width*height).skip_while(|c| *c == '2')
                .next().unwrap_or('x');
            print!("{}", res);
        }
        print!("\n");
    }
}


fn main() {
    let content = include_str!("../../input/8").trim_end();
    println!("{}", solve_a(content, 25, 6));
    solve_b(content, 25, 6);
}
