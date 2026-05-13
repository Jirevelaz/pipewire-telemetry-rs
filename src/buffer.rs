use rtrb::{Consumer, Producer, RingBuffer};

/// Abstracción del Ring Buffer SPSC para asegurar que el hilo de audio 
/// no sufra bloqueos por parte del sistema operativo o el recolector de basura.
pub fn create_bridge(capacity: usize) -> (Producer<f32>, Consumer<f32>) {
    RingBuffer::new(capacity)
}
