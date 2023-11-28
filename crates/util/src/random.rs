use essay_tensor::Tensor;


pub fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub fn random_normal() -> f32 {
    Tensor::random_normal([1], ())[0]
}

pub fn random_pareto(low: f32, high: f32, alpha: f32) -> f32 {
    let x = random();

    let h_a = high.powf(alpha);
    let l_a = low.powf(alpha);

    (- (x * h_a - x * l_a - h_a) / (h_a * l_a)).powf(- 1. / alpha)
}
