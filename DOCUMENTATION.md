# 📖 Documentation Technique — ArtistLibrePaint (Rust / GTK 4)

Cette documentation décrit l'architecture technique, le modèle d'ownership Rust et l'intégration GTK 4 / Cairo d'**ArtistLibrePaint**.

---

## 📐 1. Architecture Générale en Rust

L'application exploite les bindings officiels GTK 4 pour Rust (`gtk4-rs`) et Cairo (`cairo-rs`).

```
+-------------------------------------------------------------------+
|                     ArtistLibreApp (gtk4::Application)            |
|                                                                   |
|  +-------------------------------------------------------------+  |
|  | HeaderBar + StackSwitcher (Ruban d'outils)                  |  |
|  +-------------------------------------------------------------+  |
|                                                                   |
|  +-------------------------------------------------------------+  |
|  | Notebook (Multi-documents)                                  |  |
|  |  +-------------------------------------------------------+  |  |
|  |  | DocumentPage (Container GtkBox + Rc<RefCell<State>>)   |  |  |
|  |  |  +-------------------------------------------------+  |  |  |
|  |  |  | ScrolledWindow                                  |  |  |  |
|  |  |  |  +-------------------------------------------+  |  |  |  |
|  |  |  |  | DrawingArea                               |  |  |  |  |
|  |  |  |  | - GestureDrag (Signaux Rust)              |  |  |  |  |
|  |  |  |  | - Canvas Cairo ARgb32 (Layers)            |  |  |  |  |
|  |  |  |  +-------------------------------------------+  |  |  |  |
|  |  |  +-------------------------------------------------+  |  |  |
|  |  +-------------------------------------------------------+  |  |
|  +-------------------------------------------------------------+  |
+-------------------------------------------------------------------+
```

---

## 🦀 2. Modèle d'Ownership & Mutabilité Partagée

En Rust, la gestion des callbacks GUI nécessite le partage des données mutables entre plusieurs fermetures (closures). ArtistLibrePaint utilise le motif idiomatic Rust : **`Rc<RefCell<T>>`**.

### Structure `DocumentState`
```rust
pub struct DocumentState {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub layers: Vec<Layer>,
    pub active_layer_idx: usize,
    pub tool: Tool,
    pub brush_size: f64,
    pub brush_color: (f64, f64, f64, f64),
    pub last_x: f64,
    pub last_y: f64,
}
```

Chaque `DocumentPage` maintient un compteur de référence `Rc<RefCell<DocumentState>>`. Lors du raccordement des événements `GestureDrag` et `set_draw_func`, une référence du `Rc` est clonée (`let state_clone = state.clone()`) et déplacée (`move |_|`) dans le closure.

---

## 🧩 3. Composants et Modules

### 3.1 `src/layer.rs` — Modèle Bitmap
- `Layer` encapsule une `cairo::ImageSurface`.
- Format d'image : `cairo::Format::ARgb32` (32 bits avec canal Alpha pré-multiplié).
- Constructeur `Layer::new(name, width, height, fill_white)` gérant le remplissage initial de l'arrière-plan.

### 3.2 `src/document.rs` — Zone de Dessin & Contraintes
- Contient `DocumentPage` et la logique de capture `GestureDrag`.
- Méthode `perform_stroke` exécutant le tracé Cairo avec sélection d'opérateur :
  - `cairo::Operator::Over` pour le Pinceau.
  - `cairo::Operator::Clear` pour la Gomme.
- Méthode `export_png` qui compose l'image finale sur une surface temporaire avant écriture sur disque.

### 3.3 `src/app.rs` — Interface GTK 4
- Déclare l'application `ArtistLibreApp`.
- Construit la fenêtre principale et configure les onglets (`Fichier`, `Outils de Dessin`, `Calques`).
- Associe chaque bouton GTK aux méthodes de modification du document sélectionné dans le `Notebook`.

---

## ⚡ 4. Rendu Cairo en Rust

### Rendu Asynchrone dans `set_draw_func`
```rust
drawing_area.set_draw_func(move |_area, cr, _w, _h| {
    let st = state_clone.borrow();
    cr.set_source_rgb(0.7, 0.7, 0.7);
    let _ = cr.paint();

    for layer in &st.layers {
        if layer.visible {
            let _ = cr.set_source_surface(&layer.surface, 0.0, 0.0);
            let _ = cr.paint_with_alpha(layer.opacity);
        }
    }
});
```

Grâce à `cairo-rs`, les appels Cairo renvoient un type `Result<(), cairo::Error>`, garantissant la gestion des erreurs de rendu sans risque de crash.

---

## 🛡️ 5. Avantages de la Migration vers Rust

1. **Sécurité Mémoire Sans Garbage Collector** : Garantie à la compilation de l'absence de fuites mémoire ou d'accès concurrent non sécurisé.
2. **Performances du Binaire Natif** : Rendu et opérations matricielles compilés directement en code machine sans l'interpréteur Python.
3. **Robustesse du Rendu** : Gestion stricte des `Result` et `Option` empêchant les exceptions `NoneType` / `NullPointer`.
