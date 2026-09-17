# 🎨 ArtistLibrePaint (Version GTK 4 / Rust)

**ArtistLibrePaint** est une application open-source de dessin numérique et de retouche d'image développée en **Rust**, **GTK 4** (`gtk4-rs`) et **Cairo** (`cairo-rs`). Elle offre une sécurité mémoire garantie par Rust, des performances maximales et une architecture modulaire.

![Logo ArtistLibrePaint](logo.png)

---

## 🌟 Fonctionnalités

- 📄 **Gestion multi-documents** : Interface à onglets (`gtk4::Notebook`) avec fermeture dynamique.
- 🎨 **Outils de dessin haute performance** :
  - **Pinceau (✏️)**, **crayon**, **encre**, **marqueur**, **aérographe**, **spray**, **calligraphie** et **craie**.
  - **Gomme (🧹)** avec gestion de l'opacité et composition Alpha.
  - **Sélecteur de couleur RGBA (`gtk4::ColorButton`)**.
  - **Taille du pinceau réglable** de 1px à 100px via `gtk4::SpinButton`.
- 🥞 **Gestion des calques** :
  - Calques bitmap indépendants (`cairo::ImageSurface` ARGB32).
  - Ajout et suppression de calques dynamiques.
  - Bouton d'effacement rapide du calque actif.
- 💾 **Exportation PNG** : Sauvegarde directe des calques fusionnés.
- ⚡ **Sécurité & Rapidité Rust** : Zéro fuite mémoire, zéro segmentation fault, gestion du multithread et des références partagées via `Rc<RefCell<T>>`.

---

## 🛠️ Prérequis système

Pour compiler et exécuter ArtistLibrePaint en Rust, vous avez besoin de **Rust / Cargo** et des bibliothèques de développement GTK 4 :

### Ubuntu / Debian (Ubuntu 24.04+)
```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  cargo \
  rustc \
  libgtk-4-dev \
  libcairo2-dev \
  libgirepository-1.0-dev \
  pkg-config
```

### Fedora / RHEL
```bash
sudo dnf install -y \
  gcc \
  cargo \
  rust \
  gtk4-devel \
  cairo-devel \
  pkg-config
```

### Arch Linux
```bash
sudo pacman -S base-devel rust gtk4 cairo pkgconf
```

---

## 🚀 Compilations & Exécution

### 1. Compiler et Lancer en Mode Développement
```bash
cargo run
```

### 2. Compiler en Mode Release (Performances maximales)
```bash
cargo build --release
./target/release/artistlibre
```

### 3. Intégrer l'application au menu KDE Plasma

Après avoir installé le binaire dans un répertoire présent dans le `PATH`, copiez
le fichier `data/org.artistlibre.ArtistLibrePaint.desktop` dans
`~/.local/share/applications/`. KDE pourra alors lancer ArtistLibrePaint depuis
le menu des applications.

### 4. Choisir le backend graphique

La session GNOME utilise GTK 4. Pour utiliser le backend Qt dans KDE/Plasma,
installez les paquets de développement Qt 5 et `qmake` de votre distribution :

```bash
# Debian / Ubuntu
sudo apt-get install -y qtbase5-dev qt5-qmake

# Fedora
sudo dnf install -y qt5-qtbase-devel

# Arch Linux
sudo pacman -S qt5-base
```

Compilez ensuite avec :

```bash
cargo run --features qt
```

La détection utilise `XDG_CURRENT_DESKTOP`, `XDG_SESSION_DESKTOP` et
`DESKTOP_SESSION`. Sans la feature `qt`, une session KDE utilise automatiquement
le backend GTK comme solution de repli.

---

## 🗂️ Structure du Projet Rust

```
artistlibre/
├── Cargo.toml            # Déclarations des dépendances (gtk4, cairo-rs, glib)
├── src/
│   ├── main.rs           # Point d'entrée de l'application
│   ├── app.rs            # Interface GTK 4 (Fenêtre, HeaderBar, Ruban, Onglets)
│   ├── document.rs       # Gestion du canevas, GestureDrag et rendus Cairo
│   └── layer.rs          # Structure de calque bitmap (cairo::ImageSurface)
├── README.md             # Présentation et guide de démarrage rapide
└── DOCUMENTATION.md      # Architecture technique et guide Rust
```

---

## 📄 Licence

Ce projet est sous licence **MIT**. Libre d'utilisation, de modification et de distribution.
