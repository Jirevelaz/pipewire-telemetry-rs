pub struct SineOscillator {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl SineOscillator {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self { phase: 0.0, frequency, sample_rate }
    }

    pub fn next_chunk(&mut self, size: usize) -> Vec<f32> {
        let mut chunk = Vec::with_capacity(size);
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        
        // Modulación lenta de amplitud para simular dinamismo
        let lfo = (self.phase * 0.005).sin().abs(); 

        for _ in 0..size {
            let sample = self.phase.sin() * lfo;
            chunk.push(sample);
            self.phase += phase_increment;
        }
        chunk
    }
}
