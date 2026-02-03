pub mod empleado;
pub mod solicitud;

// Re-exportar para uso fácil
pub use empleado::Empleado;
pub use solicitud::{NuevaSolicitud, SolicitudVacaciones};
