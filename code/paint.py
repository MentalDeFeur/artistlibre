import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, Gdk, GObject

import cairo

class Layer:
    def __init__(self, name, w, h, fill_white=False):
        self.name = name
        self.visible = True
        self.opacity = 1.0
        self.surface = cairo.ImageSurface(cairo.FORMAT_ARGB32, w, h)
        if fill_white:
            cr = cairo.Context(self.surface)
            cr.set_source_rgb(1.0, 1.0, 1.0)
            cr.paint()

class DocumentPage(Gtk.Box):
    def __init__(self, title, width=700, height=500):
        super().__init__(orientation=Gtk.Orientation.VERTICAL)
        self.title = title
        self.width, self.height = width, height
        
        # Premier calque avec fond blanc
        self.layers = [Layer("Arrière-plan", width, height, fill_white=True)]
        self.active_layer_idx = 0
        
        # Paramètres de dessin par défaut
        self.tool = "brush"  # "brush" ou "eraser"
        self.brush_size = 5.0
        self.brush_color = (0.0, 0.0, 0.0, 1.0)  # RGBA
        
        # Suivi du tracé
        self.last_x = 0.0
        self.last_y = 0.0
        
        # Zone de dessin dans un ScrolledWindow
        scrolled = Gtk.ScrolledWindow()
        scrolled.set_vexpand(True)
        scrolled.set_hexpand(True)
        
        self.drawing_area = Gtk.DrawingArea()
        self.drawing_area.set_content_width(width)
        self.drawing_area.set_content_height(height)
        self.drawing_area.set_draw_func(self.on_draw)
        
        # Contrôleur de gestes pour le dessin à la souris/stylet (GTK 4)
        drag = Gtk.GestureDrag()
        drag.connect("drag-begin", self.on_drag_begin)
        drag.connect("drag-update", self.on_drag_update)
        drag.connect("drag-end", self.on_drag_end)
        self.drawing_area.add_controller(drag)
        
        scrolled.set_child(self.drawing_area)
        self.append(scrolled)

    def get_active_layer(self):
        if 0 <= self.active_layer_idx < len(self.layers):
            return self.layers[self.active_layer_idx]
        return None

    def add_layer(self, name=None):
        """Ajoute un calque transparent au document courant."""
        if name is None:
            name = f"Calque {len(self.layers) + 1}"
        new_layer = Layer(name, self.width, self.height)
        self.layers.append(new_layer)
        self.active_layer_idx = len(self.layers) - 1
        self.drawing_area.queue_draw()
        return new_layer

    def remove_active_layer(self):
        """Supprime le calque actif s'il en reste plus d'un."""
        if len(self.layers) > 1:
            self.layers.pop(self.active_layer_idx)
            if self.active_layer_idx >= len(self.layers):
                self.active_layer_idx = len(self.layers) - 1
            self.drawing_area.queue_draw()

    def clear_active_layer(self):
        """Efface le calque actif."""
        layer = self.get_active_layer()
        if layer:
            cr = cairo.Context(layer.surface)
            cr.set_operator(cairo.OPERATOR_CLEAR)
            cr.paint()
            cr.set_operator(cairo.OPERATOR_OVER)
            if self.active_layer_idx == 0:  # Si arrière-plan, remettre blanc
                cr.set_source_rgb(1.0, 1.0, 1.0)
                cr.paint()
            self.drawing_area.queue_draw()

    def draw_stroke(self, x1, y1, x2, y2):
        layer = self.get_active_layer()
        if not layer or not layer.visible:
            return

        cr = cairo.Context(layer.surface)
        cr.set_line_cap(cairo.LINE_CAP_ROUND)
        cr.set_line_join(cairo.LINE_JOIN_ROUND)
        cr.set_line_width(self.brush_size)

        if self.tool == "eraser":
            cr.set_operator(cairo.OPERATOR_CLEAR)
        else:
            cr.set_operator(cairo.OPERATOR_OVER)
            r, g, b, a = self.brush_color
            cr.set_source_rgba(r, g, b, a)

        cr.move_to(x1, y1)
        cr.line_to(x2, y2)
        cr.stroke()

        self.drawing_area.queue_draw()

    def on_drag_begin(self, gesture, start_x, start_y):
        self.last_x = start_x
        self.last_y = start_y
        self.draw_stroke(start_x, start_y, start_x, start_y)

    def on_drag_update(self, gesture, offset_x, offset_y):
        res, start_x, start_y = gesture.get_start_point()
        if not res:
            start_x, start_y = self.last_x, self.last_y
        current_x = start_x + offset_x
        current_y = start_y + offset_y
        self.draw_stroke(self.last_x, self.last_y, current_x, current_y)
        self.last_x = current_x
        self.last_y = current_y

    def on_drag_end(self, gesture, offset_x, offset_y):
        pass

    def on_draw(self, area, cr, width, height, data=None):
        # Fond de travail (fond gris neutre autour de l'image)
        cr.set_source_rgb(0.7, 0.7, 0.7)
        cr.paint()

        # Composition des calques visibles
        for layer in self.layers:
            if layer.visible:
                cr.set_source_surface(layer.surface, 0, 0)
                cr.paint_with_alpha(layer.opacity)

    def export_png(self, filepath):
        """Exporte l'image combinée au format PNG."""
        composite = cairo.ImageSurface(cairo.FORMAT_ARGB32, self.width, self.height)
        cr = cairo.Context(composite)
        for layer in self.layers:
            if layer.visible:
                cr.set_source_surface(layer.surface, 0, 0)
                cr.paint_with_alpha(layer.opacity)
        composite.write_to_png(filepath)


class ArtistLibrePaintApp(Gtk.Application):
    def __init__(self):
        super().__init__(application_id="org.example.ArtistLibrePaintApp")
        self.doc_count = 0
        self.window = None

    def do_activate(self):
        if self.window is not None:
            self.window.present()
            return

        win = Gtk.ApplicationWindow(application=self, title="ArtistLibrePaint")
        self.window = win
        win.set_default_size(1200, 800)

        main_vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
        win.set_child(main_vbox)

        # HeaderBar avec sélecteur d'onglets d'outils
        self.stack = Gtk.Stack()
        switcher = Gtk.StackSwitcher()
        switcher.set_stack(self.stack)

        header = Gtk.HeaderBar()
        header.set_title_widget(switcher)
        win.set_titlebar(header)

        # -------------------------------------------------------------
        # Onglet 1: Fichier (Nouveau, Exporter)
        # -------------------------------------------------------------
        file_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        file_box.set_margin_start(10)
        file_box.set_margin_end(10)
        file_box.set_margin_top(5)
        file_box.set_margin_bottom(5)

        btn_new = Gtk.Button(label="[+] Nouveau Document")
        btn_new.connect("clicked", lambda b: self.add_tab())
        file_box.append(btn_new)

        btn_export = Gtk.Button(label="💾 Exporter en PNG")
        btn_export.connect("clicked", self.on_export_clicked)
        file_box.append(btn_export)

        self.stack.add_titled(file_box, "file", "Fichier")

        # -------------------------------------------------------------
        # Onglet 2: Outils de dessin (Pinceau, Gomme, Couleur, Taille)
        # -------------------------------------------------------------
        tools_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        tools_box.set_margin_start(10)
        tools_box.set_margin_end(10)
        tools_box.set_margin_top(5)
        tools_box.set_margin_bottom(5)

        # Choix de l'outil
        lbl_tool = Gtk.Label(label="Outil :")
        tools_box.append(lbl_tool)

        btn_brush = Gtk.Button(label="✏️ Pinceau")
        btn_brush.connect("clicked", lambda b: self.set_tool("brush"))
        tools_box.append(btn_brush)

        btn_eraser = Gtk.Button(label="🧹 Gomme")
        btn_eraser.connect("clicked", lambda b: self.set_tool("eraser"))
        tools_box.append(btn_eraser)

        # Sélecteur de couleur
        lbl_color = Gtk.Label(label="Couleur :")
        tools_box.append(lbl_color)

        color_btn = Gtk.ColorButton()
        rgba = Gdk.RGBA()
        rgba.red, rgba.green, rgba.blue, rgba.alpha = 0.0, 0.0, 0.0, 1.0
        color_btn.set_rgba(rgba)
        color_btn.connect("color-set", self.on_color_changed)
        tools_box.append(color_btn)

        # Taille du pinceau
        lbl_size = Gtk.Label(label="Taille :")
        tools_box.append(lbl_size)

        adjustment = Gtk.Adjustment(value=5, lower=1, upper=100, step_increment=1)
        spin_size = Gtk.SpinButton(adjustment=adjustment, numeric=True)
        spin_size.connect("value-changed", self.on_size_changed)
        tools_box.append(spin_size)

        # Effacer calque
        btn_clear = Gtk.Button(label="🗑️ Effacer le calque")
        btn_clear.connect("clicked", self.on_clear_layer_clicked)
        tools_box.append(btn_clear)

        self.stack.add_titled(tools_box, "tools", "Outils de Dessin")

        # -------------------------------------------------------------
        # Onglet 3: Calques (Ajouter, Supprimer, Sélection)
        # -------------------------------------------------------------
        layer_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        layer_box.set_margin_start(10)
        layer_box.set_margin_end(10)
        layer_box.set_margin_top(5)
        layer_box.set_margin_bottom(5)

        btn_add_layer = Gtk.Button(label="[+] Nouveau calque")
        btn_add_layer.connect("clicked", self.add_layer_to_current_document)
        layer_box.append(btn_add_layer)

        btn_del_layer = Gtk.Button(label="[-] Supprimer calque")
        btn_del_layer.connect("clicked", self.remove_layer_from_current_document)
        layer_box.append(btn_del_layer)

        self.lbl_layer_info = Gtk.Label(label="Calque actif : Arrière-plan")
        layer_box.append(self.lbl_layer_info)

        self.stack.add_titled(layer_box, "layer", "Calques")

        # Ajouter la barre d'outils au-dessus du Canvas
        main_vbox.append(self.stack)

        # Onglets multi-documents
        self.notebook = Gtk.Notebook()
        self.notebook.set_vexpand(True)
        self.notebook.connect("switch-page", self.on_tab_changed)
        main_vbox.append(self.notebook)

        self.add_tab()
        win.present()

    def get_current_document(self):
        page_idx = self.notebook.get_current_page()
        if page_idx != -1:
            doc = self.notebook.get_nth_page(page_idx)
            if isinstance(doc, DocumentPage):
                return doc
        return None

    def add_tab(self):
        self.doc_count += 1
        title = f"Image_{self.doc_count}.png"
        doc = DocumentPage(title)

        tab_hbox = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        tab_hbox.append(Gtk.Label(label=title))
        close_btn = Gtk.Button(label="✕")
        close_btn.set_has_frame(False)
        tab_hbox.append(close_btn)

        page_idx = self.notebook.append_page(doc, tab_hbox)
        self.notebook.set_tab_reorderable(doc, True)
        close_btn.connect("clicked", self.close_document, doc)
        self.notebook.set_current_page(page_idx)
        self.update_layer_info()

    def set_tool(self, tool_name):
        doc = self.get_current_document()
        if doc:
            doc.tool = tool_name

    def on_color_changed(self, color_btn):
        rgba = color_btn.get_rgba()
        doc = self.get_current_document()
        if doc:
            doc.brush_color = (rgba.red, rgba.green, rgba.blue, rgba.alpha)

    def on_size_changed(self, spin_btn):
        val = spin_btn.get_value()
        doc = self.get_current_document()
        if doc:
            doc.brush_size = val

    def on_clear_layer_clicked(self, button):
        doc = self.get_current_document()
        if doc:
            doc.clear_active_layer()

    def add_layer_to_current_document(self, button):
        doc = self.get_current_document()
        if doc:
            doc.add_layer()
            self.update_layer_info()

    def remove_layer_from_current_document(self, button):
        doc = self.get_current_document()
        if doc:
            doc.remove_active_layer()
            self.update_layer_info()

    def on_tab_changed(self, notebook, page, page_num):
        self.update_layer_info()

    def update_layer_info(self):
        doc = self.get_current_document()
        if doc and hasattr(self, "lbl_layer_info"):
            layer = doc.get_active_layer()
            if layer:
                self.lbl_layer_info.set_label(f"Calque actif ({doc.active_layer_idx + 1}/{len(doc.layers)}) : {layer.name}")
            else:
                self.lbl_layer_info.set_label("Aucun calque actif")

    def on_export_clicked(self, button):
        doc = self.get_current_document()
        if not doc:
            return
        
        # Export rapide sous format PNG
        filename = f"{doc.title}"
        doc.export_png(filename)
        print(f"Document exporté sous {filename}")

    def close_document(self, button, document):
        page_idx = self.notebook.page_num(document)
        if page_idx != -1:
            self.notebook.remove_page(page_idx)
        self.update_layer_info()

if __name__ == "__main__":
    app = ArtistLibrePaintApp()
    app.run()







