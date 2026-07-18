# Lab 1 — Relleno de polígonos

Laboratorio de Gráficas por Computadora. Implementa un algoritmo de relleno de
polígonos por **scan-line** con la **regla par-impar** (even-odd), escrito en
Rust y sin dependencias externas (el PNG se genera con un encoder propio).

## Qué hace

- Dibuja y rellena 5 polígonos, cada uno con su color de relleno y de línea:
  - **Polígono 1** (estrella, 10 vértices) — amarillo
  - **Polígono 2** (cuadrilátero) — azul
  - **Polígono 3** (triángulo) — rojo
  - **Polígono 4** (tetera, 18 vértices) — verde
  - **Polígono 5** — agujero dentro del polígono 4, no se rellena
- El agujero se logra pasando ambos contornos al mismo relleno par-impar:
  las intersecciones del contorno interior alternan la paridad y dejan la
  región sin pintar.
- Las líneas se trazan con el algoritmo de Bresenham.
- El resultado se guarda en `out.png` (800×450, origen abajo-izquierda).

## Cómo ejecutar

```sh
cargo run --release
```

Genera `out.png` en la raíz del proyecto.

## Resultado

![out](out.png)
