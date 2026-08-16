# 🎨 ArtistLibrePaint

**ArtistLibrePaint** est une application open-source de dessin numérique et de retouche d'image basée sur **Python 3**, **GTK 4** (`PyGObject`) et **Cairo**. Elle offre une interface moderne à onglets, une gestion multi-calques et des outils de dessin fluides.

---

## 🌟 Fonctionnalités

- 📄 **Gestion multi-documents** : Ouvrez et travaillez sur plusieurs images simultanément via des onglets interactifs.
- 🎨 **Outils de dessin en temps réel** :
  - **Pinceau (✏️)** et **Gomme (🧹)** avec gestion de l'opacité et de la composition.
  - **Sélecteur de couleur RGBA (`Gtk.ColorButton`)** pour une sélection précise.
  - **Taille du pinceau réglable** de 1px à 100px.
- 🥞 **Gestion des calques** :
  - Calques illimités avec fond transparent ou blanc.
  - Ajout et suppression de calques.
  - Composition en temps réel avec fusion alpha via Cairo.
  - Bouton d'effacement rapide du calque actif.
- 💾 **Exportation** : Exportez vos créations instantanément au format PNG.
- 🖥️ **Interface réactive (GTK 4)** : Layout fluide avec barres d'outils rétractables et défilement adaptatif (`Gtk.ScrolledWindow`).

---

## 🛠️ Prérequis système

L'application repose sur GTK 4 et Cairo. Assurez-vous d'installer les bibliothèques système nécessaires :

### Ubuntu / Debian (Ubuntu 24.04+)
```bash
sudo apt-get update
sudo apt-get install -y \
  python3-gi \
  python3-gi-cairo \
  gir1.2-gtk-4.0 \
  libgirepository-2.0-dev \
  libcairo2-dev \
  pkg-config \
  python3-dev
```

### Fedora / RHEL
```bash
sudo dnf install -y \
  python3-gobject \
  gtk4 \
  cairo-devel \
  gobject-introspection-devel \
  pkg-config
```

### Arch Linux
```bash
sudo pacman -S gtk4 python-gobject cairo pkgconf
```

---

## 🚀 Installation

### 1. Cloner le projet
```bash
git clone https://github.com/votre-compte/artistlibre.git
cd artistlibre
```

### 2. Créer l'environnement virtuel avec les packages système
Pour utiliser la liaison système `gi` (PyGObject) nativement :

```bash
# Création de l'environnement virtuel avec accès aux libs système
python3 -m venv env --system-site-packages

# Activation de l'environnement
source env/bin/activate
```

### 3. Installer les dépendances Python
```bash
pip install PyGObject pycairo
```

---

## 💻 Utilisation

Pour lancer l'application :

```bash
# S'assurer que le venv est activé
source env/bin/activate

# Lancer ArtistLibrePaint
python3 code/init.py
```

---

## 🗂️ Structure du projet

```
artistlibre/
├── code/
│   └── init.py           # Point d'entrée et code source principal
├── README.md             # Présentation et guide de démarrage rapide
└── DOCUMENTATION.md      # Architecture technique et guide du développeur
```

---

## 📖 En savoir plus

Pour une description détaillée des classes, de la gestion des événements GTK 4 et du rendu Cairo, consultez la [Documentation Technique](DOCUMENTATION.md).

---

## 📄 Licence

Ce projet est sous licence **MIT**. Libre d'utilisation, de modification et de distribution.
