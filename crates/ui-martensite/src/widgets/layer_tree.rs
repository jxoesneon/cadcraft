//! Layer list widget: on/off, freeze and lock flags per CAD layer.

#[derive(Clone, Debug, PartialEq)]
pub struct LayerItemDef {
    pub id: u64,
    pub name: String,
    /// Light-bulb on/off.
    pub visible: bool,
    /// Sun/snowflake freeze flag (not regenerated while frozen).
    pub frozen: bool,
    pub locked: bool,
    /// Layer colour chip.
    pub color: [u8; 3],
    /// True while this is the drawing's current layer.
    pub is_current: bool,
    /// Nested xref/group rows.
    pub children: Vec<LayerItemDef>,
}

pub struct LayerTreeWidget {
    pub layers: Vec<LayerItemDef>,
    pub selected_layer_id: Option<u64>,
    pub current_layer_id: Option<u64>,
}

impl LayerTreeWidget {
    pub fn new() -> Self {
        Self { layers: Vec::new(), selected_layer_id: None, current_layer_id: None }
    }

    pub fn select_layer(&mut self, id: u64) {
        self.selected_layer_id = Some(id);
    }

    /// Make a layer current; clears the flag on the previous current layer.
    pub fn set_current(&mut self, id: u64) {
        clear_current(&mut self.layers);
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.is_current = true;
        }
        self.current_layer_id = Some(id);
    }

    pub fn toggle_visibility(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.visible = !item.visible;
        }
    }

    pub fn toggle_frozen(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.frozen = !item.frozen;
        }
    }

    pub fn toggle_lock(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.locked = !item.locked;
        }
    }
}

impl Default for LayerTreeWidget {
    fn default() -> Self {
        Self::new()
    }
}

fn find_layer_mut(items: &mut [LayerItemDef], id: u64) -> Option<&mut LayerItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_layer_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

fn clear_current(items: &mut [LayerItemDef]) {
    for item in items.iter_mut() {
        item.is_current = false;
        clear_current(&mut item.children);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(id: u64, name: &str) -> LayerItemDef {
        LayerItemDef {
            id,
            name: name.to_string(),
            visible: true,
            frozen: false,
            locked: false,
            color: [255, 255, 255],
            is_current: false,
            children: vec![],
        }
    }

    #[test]
    fn test_layer_tree_mutation() {
        let mut tree = LayerTreeWidget::new();
        tree.layers.push(layer(1, "0"));
        tree.layers.push(layer(2, "Walls"));

        tree.select_layer(2);
        assert_eq!(tree.selected_layer_id, Some(2));

        tree.set_current(2);
        assert_eq!(tree.current_layer_id, Some(2));
        assert!(tree.layers[1].is_current);

        tree.set_current(1);
        assert!(!tree.layers[1].is_current);
        assert!(tree.layers[0].is_current);

        tree.toggle_visibility(2);
        assert!(!tree.layers[1].visible);
        tree.toggle_visibility(2);
        assert!(tree.layers[1].visible);

        tree.toggle_frozen(2);
        assert!(tree.layers[1].frozen);

        tree.toggle_lock(2);
        assert!(tree.layers[1].locked);
    }
}
