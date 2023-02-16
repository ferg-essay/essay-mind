pub fn power_msq(data: &[f32]) -> f32 {
    let sum: f32 = data.iter().map(|x| x * x).sum();
    
    sum / data.len() as f32
}

pub fn power_rms(data: &[f32]) -> f32 {
    let sum: f32 = data.iter().map(|x| x * x).sum();
    
    (sum / data.len() as f32).sqrt()
}