pub mod buffer;
pub mod dsp;
pub mod ffi;

use tokio::sync::mpsc;
use std::time::Duration;

/// Eventos de telemetría enviados hacia la interfaz de usuario o cliente.
pub enum AudioEvent {
    Rms(f32),
}

pub struct Engine {
    rx: mpsc::Receiver<AudioEvent>,
}

impl Engine {
    /// Inicia el motor en modo "Mock" (Simulación).
    /// Permite probar la lógica DSP y el puente sin dependencias de hardware.
    pub async fn start_mock() -> anyhow::Result<Self> {
        // 1. Crear el puente lock-free (capacidad de 8192 muestras flotantes)
        let (producer, mut consumer) = buffer::create_bridge(8192);

        // 2. Iniciar el hilo de audio simulado (FFI)
        ffi::start_mock_audio_thread(producer);

        // 3. Canal para comunicación asíncrona
        let (tx, rx) = mpsc::channel(128);

        // 4. Actor Consumidor: Drena el puente y procesa matemáticas
        tokio::spawn(async move {
            let mut local_buf = Vec::with_capacity(2048);
            
            loop {
                // Extraer muestras del puente sin bloquear el hilo de audio
                while let Ok(sample) = consumer.pop() {
                    local_buf.push(sample);
                }

                if !local_buf.is_empty() {
                    // Procesamiento DSP puro
                    let rms = dsp::rms::calculate_rms(&local_buf);
                    
                    // Notificar al suscriptor
                    let _ = tx.send(AudioEvent::Rms(rms)).await;
                    local_buf.clear();
                }
                
                // Frecuencia de actualización de la UI (~60 FPS)
                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        });

        Ok(Self { rx })
    }

    pub fn subscribe(self) -> mpsc::Receiver<AudioEvent> {
        self.rx
    }
}
