use rand::Rng;

fn approximate_pi(num_hits: usize) -> f64 {
  let mut rng = rand::thread_rng();
  
  let circle_radius: f64 = 1.0;
  let rectangle_area: f64 = 4.0 * circle_radius * circle_radius;
  let circle_area: f64 = std::f64::consts::PI * circle_radius * circle_radius;

  let hits_rectangle = num_hits;
  let mut hits_circle = 0;
  
  for _ in 0..num_hits {
    if rng.gen::<f64>() * rectangle_area <= circle_area {
      hits_circle += 1;
    }
  }
  
  4.0 * (hits_circle as f64 / hits_rectangle as f64)
}

fn main() {
  let appr_pi = approximate_pi(15_000_000);
  
  println!("PI ~= {}", appr_pi);
  println!("The approximation is:\n{}%\n off the true value of\n{}.", ((appr_pi - std::f64::consts::PI).abs() / std::f64::consts::PI) * 100.0, std::f64::consts::PI);
}
