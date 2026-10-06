# Pluma

Editor vectorial de Nodalix.

El motor de este repositorio es el código de [storytold/vectorcraft](https://github.com/storytold/vectorcraft) en `571881a830830bf8f221aa8e2635eb999db90708`.
La licencia de ese código se conserva, igual que `NOTICE` y `docs/upstream-readme.md`.
Pluma es el nombre del producto. El árbol anterior, basado en Graphite, deja de ser el motor.
Un sync posterior compara commits de VectorCraft. No se sustituye este árbol con un reset.

## Compilar

```bash
cargo run -p vectorcraft
```

El binario de escritorio se llama `pluma`.

## Estado

La shell que entra en este import es la de upstream. La interfaz GTK 4 / libadwaita de Nodalix no está cableada todavía. El motor sí está en este repositorio.
