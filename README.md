
# Tamagotchi en Rust 🐾

Un simulador de mascota virtual implementado en Rust que simula el clásico juego de Tamagotchi.

## Descripción

Este proyecto es una implementación del famoso juego Tamagotchi donde debes cuidar de tu mascota virtual. La mascota tiene estados de hambre y felicidad que cambian con el tiempo, y debes interactuar con ella para mantenerla feliz y bien alimentada.

## Características

- **Mascota virtual interactiva**: Tu Tamagotchi tiene nombre, niveles de hambre y felicidad
- **Sistema de estado en tiempo real**: Los estados cambian automáticamente cada segundo
- **Acciones disponibles**:
  - 🎮 Jugar con tu Tamagotchi (aumenta felicidad)
  - 🍎 Alimentar a tu Tamagotchi (reduce hambre)
  - 👋 Salir del juego
- **Interfaz en consola**: Menú interactivo y estado actualizado en tiempo real
- **Sistema de hilos**: Actualización automática del estado sin bloquear la entrada del usuario

## Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada y lógica principal
├── constants.rs         # Constantes del juego (valores iniciales, incrementos, etc.)
├── utils.rs            # Utilidades (entrada de usuario, limpieza de pantalla)
└── models/
    ├── mod.rs          # Módulo de modelos
    └── tamagotchi.rs   # Estructura y lógica del Tamagotchi
```

## Instalación y Ejecución

### Requisitos
- Rust (edición 2021 o superior)
- Cargo

### Ejecutar el proyecto
```bash
cargo run
```

## Cómo Jugar

1. **Inicio**: Introduce el nombre de tu Tamagotchi
2. **Menú principal**: Verás tres opciones:
   - `1` - Jugar con tu Tamagotchi
   - `2` - Alimentar a tu Tamagotchi  
   - `3` - Salir del juego
3. **Estado**: El estado de tu mascota se actualiza automáticamente cada segundo
4. **Cuidado**: Mantén a tu Tamagotchi feliz jugando con él y aliméntalo cuando tenga hambre

## Mecánicas del Juego

### Estados
- **Felicidad**: Comienza en 100, disminuye cuando el Tamagotchi tiene mucha hambre
- **Hambre**: Comienza en 0, aumenta gradualmente con el tiempo

### Acciones
- **Jugar**: Aumenta la felicidad en 10 puntos y puede aumentar ligeramente el hambre
- **Alimentar**: Reduce el hambre en 10 puntos
- **Tiempo**: Cada segundo aumenta el hambre en 1 punto

### Alertas
- Cuando el hambre supera 80 puntos, tu Tamagotchi estará hambriento y su felicidad comenzará a disminuir

## Configuración

Puedes modificar los valores del juego editando las constantes en `src/constants.rs`:

```rust
pub const INITIAL_HUNGER: u32 = 0;           // Hambre inicial
pub const INITIAL_HAPPINESS: u32 = 100;     // Felicidad inicial
pub const HUNGER_WARNING: u32 = 80;         // Umbral de hambre crítica
pub const PLAY_HAPPINESS_INCREASE: u32 = 10; // Aumento de felicidad al jugar
// ... más constantes
```

## Arquitectura Técnica

### Concurrencia
El proyecto utiliza `Arc<Mutex<T>>` para compartir el estado del Tamagotchi entre hilos de manera segura:
- **Hilo principal**: Maneja la entrada del usuario
- **Hilo secundario**: Actualiza el estado cada segundo y muestra la interfaz

### Modules
- `models::tamagotchi`: Contiene la estructura `Tamagotchi` y sus métodos
- `utils`: Funciones de utilidad para entrada/salida
- `constants`: Valores constantes del juego

## Contribuir

Si quieres contribuir al proyecto:

1. Haz un fork del repositorio
2. Crea una rama para tu función (`git checkout -b nueva-funcion`)
3. Haz commit de tus cambios (`git commit -am 'Agregar nueva función'`)
4. Push a la rama (`git push origin nueva-funcion`)
5. Crea un Pull Request

## Futuras Mejoras

- [ ] Sistema de guardado/carga
- [ ] Múltiples tipos de comida
- [ ] Diferentes actividades de juego
- [ ] Sistema de envejecimiento
- [ ] Interfaz gráfica
- [ ] Sonidos y efectos

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo `LICENSE` para más detalles.

---

¡Disfruta cuidando de tu Tamagotchi virtual! 🎮✨
