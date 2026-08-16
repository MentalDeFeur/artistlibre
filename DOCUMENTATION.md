# 📖 Documentation Technique — ArtistLibrePaint

Cette documentation est destinée aux développeurs souhaitant comprendre l'architecture, le fonctionnement interne et les choix d'implémentation d'**ArtistLibrePaint**.

---

## 📐 1. Architecture Globale

L'architecture repose sur l'écosystème **GNOME / GTK 4** et le moteur graphique matriciel 2D **Cairo**.

```
+-------------------------------------------------------------------+
|                    ArtistLibrePaintApp (Gtk.Application)          |
|                                                                   |
|  +-------------------------------------------------------------+  |
|  | HeaderBar + Gtk.StackSwitcher (Ruban d'outils)             |  |
|  | - Fichier | - Outils de Dessin | - Calques                  |  |
|  +-------------------------------------------------------------+  |
|                                                                   |
|  +-------------------------------------------------------------+  |
|  | Gtk.Notebook (Multi-documents)                              |  |
|  |  +-------------------------------------------------------+  |  |
|  |  | DocumentPage (Gtk.Box)                                |  |  |
|  |  |  +-------------------------------------------------+  |  |  |
|  |  |  | Gtk.ScrolledWindow                              |  |  |  |
|  |  |  |  +-------------------------------------------+  |  |  |  |
|  |  |  |  | Gtk.DrawingArea                           |  |  |  |  |
|  |  |  |  | - Gtk.GestureDrag (Capture de la souris)    |  |  |  |  |
|  |  |  |  | - Surface Cairo composée (Layers)          |  |  |  |  |
|  |  |  |  +-------------------------------------------+  |  |  |  |
|  |  |  +-------------------------------------------------+  |  |  |
|  |  +-------------------------------------------------------+  |  |
|  +-------------------------------------------------------------+  |
+-------------------------------------------------------------------+
```

---

## 🧩 2. Description des Classes

### 2.1 `Layer`
Représente un calque individuel au sein d'un document.

- **Attributs** :
  - `name` *(str)* : Nom du calque (ex: `"Arrière-plan"`, `"Calque 2"`).
  - `visible` *(bool)* : Booléen indiquant si le calque est affiché lors du rendu.
  - `opacity` *(float)* : Opacité du calque entre `0.0` (invisible) et `1.0` (opaque).
  - `surface` *(cairo.ImageSurface)* : Surface Cairo au format `cairo.FORMAT_ARGB32` contenant les pixels du calque.
- **Initialisation** :
  ```python
  Layer(name, w, h, fill_white=False)
  ```
  Si `fill_white=True`, le calque est pré-rempli avec un fond blanc opaque (utilisé pour le calque `"Arrière-plan"`).

---

### 2.2 `DocumentPage`
Hérite de `Gtk.Box`. Représente la zone de travail d'un document ouvert (un onglet du `Gtk.Notebook`).

- **Attributs d'état** :
  - `layers` *(list[Layer])* : Liste ordonnée des calques du document (du bas vers le haut).
  - `active_layer_idx` *(int)* : Index du calque actuellement sélectionné pour le dessin.
  - `tool` *(str)* : Outil actif (`"brush"` ou `"eraser"`).
  - `brush_size` *(float)* : Épaisseur du pinceau en pixels.
  - `brush_color` *(tuple[float, float, float, float])* : Couleur RGBA entre `0.0` et `1.0`.
  - `drawing_area` *(Gtk.DrawingArea)* : Composant GTK 4 sur lequel s'effectue l'affichage.

- **Méthodes Principales** :
  - `get_active_layer()` : Renvoie l'instance du `Layer` actif.
  - `add_layer(name=None)` : Crée et ajoute un nouveau calque transparent au-dessus de la pile.
  - `remove_active_layer()` : Supprime le calque actif (si plus d'un calque existe).
  - `clear_active_layer()` : Réinitialise les pixels du calque actif.
  - `draw_stroke(x1, y1, x2, y2)` : Trace un segment entre `(x1, y1)` et `(x2, y2)` sur le calque actif avec l'outil et les paramètres actuels.
  - `on_draw(area, cr, width, height, data=None)` : Callback appelé par GTK pour restituer le canevas à l'écran.
  - `export_png(filepath)` : Combine l'ensemble des calques visibles sur une nouvelle surface et l'enregistre au format PNG.

---

### 2.3 `ArtistLibrePaintApp`
Hérite de `Gtk.Application`. Gère le cycle de vie de l'application, la fenêtre principale (`Gtk.ApplicationWindow`), la barre d'outils supérieure et les onglets.

- **Composants d'interface** :
  - `stack` *(Gtk.Stack)* & `switcher` *(Gtk.StackSwitcher)* : Implémentent le ruban d'onglets d'outils (Fichier, Outils de Dessin, Calques).
  - `notebook` *(Gtk.Notebook)* : Gestionnaire multi-documents.
- **Gestionnaires d'événements (Callbacks)** :
  - `add_tab()` : Ouvre un nouvel onglet avec un `DocumentPage`.
  - `close_document(button, document)` : Ferme l'onglet spécifié.
  - `on_color_changed(color_btn)` : Synchronise la couleur sélectionnée avec le document actif.
  - `on_size_changed(spin_btn)` : Synchronise la taille du pinceau avec le document actif.
  - `set_tool(tool_name)` : Bascule entre `"brush"` et `"eraser"`.

---

## 🖱️ 3. Modèle d'Événements GTK 4

En GTK 4, la gestion des entrées utilisateur s'effectue via des **Event Controllers**.

### Utilisation de `Gtk.GestureDrag`
Dans `DocumentPage.__init__` :

```python
drag = Gtk.GestureDrag()
drag.connect("drag-begin", self.on_drag_begin)
drag.connect("drag-update", self.on_drag_update)
drag.connect("drag-end", self.on_drag_end)
self.drawing_area.add_controller(drag)
```

1. **`drag-begin`** : Déclenché lors du premier clic sur le canevas. Initialise `(last_x, last_y)` et dessine un premier point/point mort.
2. **`drag-update`** : Déclenché continuellement lorsque l'utilisateur déplace le curseur en maintenant le clic.
   - Calcule la position actuelle : `current_x = start_x + offset_x`.
   - Appelle `draw_stroke(last_x, last_y, current_x, current_y)`.
   - Met à jour `last_x, last_y`.
3. **`drag-end`** : Déclenché au relâchement du bouton.

---

## 🎨 4. Compositing et Rendu Cairo

### Pipeline d'affichage (`on_draw`)
Lorsque `drawing_area.queue_draw()` est appelé, GTK invoque `on_draw` avec le contexte de dessin Cairo `cr` :

1. **Fond neutre** : Peinture d'un arrière-plan gris (`#B3B3B3`) représentant l'espace de travail hors canevas.
2. **Boucle de composition** :
   ```python
   for layer in self.layers:
       if layer.visible:
           cr.set_source_surface(layer.surface, 0, 0)
           cr.paint_with_alpha(layer.opacity)
   ```
   Chaque surface de calque est superposée en utilisant la formule de composition Alpha Blend.

### Pinceau vs Gomme
- **Mode Pinceau** (`OPERATOR_OVER`) :
  ```python
  cr.set_operator(cairo.OPERATOR_OVER)
  cr.set_source_rgba(r, g, b, a)
  ```
  Applique la nouvelle couleur par-dessus les pixels existants du calque.

- **Mode Gomme** (`OPERATOR_CLEAR`) :
  ```python
  cr.set_operator(cairo.OPERATOR_CLEAR)
  ```
  Remet les pixels touchés à la valeur RGBA `(0, 0, 0, 0)` (transparence totale).

---

## 🚀 5. Perspectives d'Évolution

Voici les pistes d'amélioration suggérées pour les futures versions d'ArtistLibrePaint :

1. **Système d'Annulation / Rétablissement (Undo/Redo)** :
   - Conserver un historique des états de `cairo.ImageSurface` ou des commandes de tracé.
2. **Formes géométriques & Outils avancés** :
   - Ajout d'outils Ligne, Rectangle, Ellipse et Pot de peinture (Remplissage par inondation / Flood Fill).
3. **Opacité et modes de fusion des calques** :
   - Ajouter un curseur d'opacité et un menu déroulant pour les modes de fusion Cairo (`OPERATOR_MULTIPLY`, `OPERATOR_SCREEN`, etc.).
4. **Boîte de dialogue de sauvegarde native** :
   - Remplacer l'exportation automatique par un `Gtk.FileDialog` pour choisir l'emplacement et le nom du fichier PNG.
