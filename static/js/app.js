// JavaScript básico para mejorar la experiencia del usuario
console.log('🦀 Sistema de Vacaciones - Powered by Rust + Axum');

// Agregar animaciones sutiles cuando se carga la página
document.addEventListener('DOMContentLoaded', () => {
    console.log('✅ Página cargada');
    
    // Animar las cards al hacer scroll (opcional)
    const cards = document.querySelectorAll('.card, .empleado-card');
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '0';
                entry.target.style.transform = 'translateY(20px)';
                
                setTimeout(() => {
                    entry.target.style.transition = 'opacity 0.5s, transform 0.5s';
                    entry.target.style.opacity = '1';
                    entry.target.style.transform = 'translateY(0)';
                }, 100);
                
                observer.unobserve(entry.target);
            }
        });
    }, { threshold: 0.1 });
    
    cards.forEach(card => observer.observe(card));
});

// Función para hacer peticiones a la API (útil para futuras funcionalidades)
async function fetchAPI(endpoint) {
    try {
        const response = await fetch(`/api${endpoint}`);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    } catch (error) {
        console.error('Error fetching API:', error);
        return null;
    }
}

// Exportar para uso en otros scripts si es necesario
window.vacacionesApp = {
    fetchAPI
};
