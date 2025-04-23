use std::arch::x86_64::*;

pub struct SimdVector<T> {
    data: Vec<T>,
}

impl SimdVector<f32> {
    pub fn new(capacity: usize) -> Self {
        SimdVector {
            data: Vec::with_capacity(capacity),
        }
    }
    
    #[cfg(target_feature = "avx")]
    pub unsafe fn sum(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        
        let mut sum = _mm256_setzero_ps();
        let chunks = self.data.chunks_exact(8);
        let remainder = chunks.remainder();
        
        for chunk in chunks {
            let chunk_ptr = chunk.as_ptr() as *const __m256;
            let chunk_data = _mm256_loadu_ps(chunk_ptr);
            sum = _mm256_add_ps(sum, chunk_data);
        }
        
        let mut result = 0.0;
        let sum_array = std::mem::transmute::<__m256, [f32; 8]>(sum);
        for val in sum_array.iter() {
            result += val;
        }
        
        for val in remainder {
            result += *val;
        }
        
        result
    }
}