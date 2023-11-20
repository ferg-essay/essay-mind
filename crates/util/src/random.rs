use essay_tensor::Tensor;


pub fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub fn random_normal() -> f32 {
    Tensor::random_normal([1], ())[0]
}
