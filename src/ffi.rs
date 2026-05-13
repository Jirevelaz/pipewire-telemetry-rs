use crate::dsp::generator::SineOscillator;
use rtrb::Producer;
use std::time::Duration;

/// Módulo destinado a contener el código unsafe y llamadas a PipeWire.
/// Actualmente simula el comportamiento de un driver de audio.
pub fn start_mock_audio_thread(mut producer: Producer<f32>) {
    std::thread::spawn(move || {
        let mut osc = SineOscillator::new(440.0, 48000.0);
        
        loop {
            // Simula la llegada de un bloque de 512 muestras desde el hardware
            let chunk = osc.next_chunk(512);
            
            for &sample in &chunk {
                // Operación atómica: si el búfer está lleno, se descarta el dato
                let _ = producer.push(sample); 
            }
            
            // Simulación de latencia de hardware (10ms)
            std::thread::sleep(Duration::from_millis(10));
        }
    });
}
