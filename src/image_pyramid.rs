pub struct ImagePyramid {
    pub image: FloatImage,
    pub current_scale: usize,
    pub min_scale: usize,
    pub max_scale: usize,
}
