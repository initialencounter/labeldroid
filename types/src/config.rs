#[cfg(feature = "napi-support")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[cfg_attr(feature = "napi-support", napi(js_name = "auto_save"))]
    pub auto_save: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "display_label_popup"))]
    pub display_label_popup: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "with_image_data"))]
    pub with_image_data: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "keep_prev"))]
    pub keep_prev: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "keep_prev_scale"))]
    pub keep_prev_scale: bool,
    #[cfg_attr(
        feature = "napi-support",
        napi(js_name = "keep_prev_brightness_contrast")
    )]
    pub keep_prev_brightness_contrast: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "logger_level"))]
    pub logger_level: String,

    #[cfg_attr(feature = "napi-support", napi(js_name = "flags"))]
    pub flags: Option<String>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "label_flags"))]
    pub label_flags: Option<String>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "labels"))]
    pub labels: Option<Vec<String>>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "file_search"))]
    pub file_search: Option<String>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "sort_labels"))]
    pub sort_labels: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "validate_label"))]
    pub validate_label: Option<String>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "default_shape_color"))]
    pub default_shape_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "shape_color"))]
    pub shape_color: Option<String>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "shift_auto_shape_color"))]
    pub shift_auto_shape_color: i32,
    #[cfg_attr(feature = "napi-support", napi(js_name = "label_colors"))]
    pub label_colors: Option<String>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "shape"))]
    pub shape: ShapeConfig,
    #[cfg_attr(feature = "napi-support", napi(js_name = "ai"))]
    pub ai: AiConfig,

    #[cfg_attr(feature = "napi-support", napi(js_name = "flag_dock"))]
    pub flag_dock: DockConfig,
    #[cfg_attr(feature = "napi-support", napi(js_name = "label_dock"))]
    pub label_dock: DockConfig,
    #[cfg_attr(feature = "napi-support", napi(js_name = "shape_dock"))]
    pub shape_dock: DockConfig,
    #[cfg_attr(feature = "napi-support", napi(js_name = "file_dock"))]
    pub file_dock: DockConfig,

    #[cfg_attr(feature = "napi-support", napi(js_name = "show_label_text_field"))]
    pub show_label_text_field: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "label_completion"))]
    pub label_completion: String,
    #[cfg_attr(feature = "napi-support", napi(js_name = "fit_to_content"))]
    pub fit_to_content: FitToContent,

    #[cfg_attr(feature = "napi-support", napi(js_name = "epsilon"))]
    pub epsilon: f64,
    #[cfg_attr(feature = "napi-support", napi(js_name = "canvas"))]
    pub canvas: CanvasConfig,

    #[cfg_attr(feature = "napi-support", napi(js_name = "shortcuts"))]
    pub shortcuts: Shortcuts,
}

impl Default for Config {
    fn default() -> Self {
        // shortcuts defaults based on default_config.yaml
        let shortcuts = Shortcuts::default();

        Config {
            auto_save: true,
            display_label_popup: true,
            with_image_data: false,
            keep_prev: false,
            keep_prev_scale: false,
            keep_prev_brightness_contrast: false,
            logger_level: "info".to_string(),

            flags: None,
            label_flags: None,
            labels: None,
            file_search: None,
            sort_labels: true,
            validate_label: None,

            default_shape_color: vec![0, 255, 0],
            shape_color: Some("auto".to_string()),
            shift_auto_shape_color: 0,
            label_colors: None,

            shape: ShapeConfig::default(),
            ai: AiConfig::default(),

            flag_dock: DockConfig::default(),
            label_dock: DockConfig::default(),
            shape_dock: DockConfig::default(),
            file_dock: DockConfig::default(),

            show_label_text_field: true,
            label_completion: "startswith".to_string(),
            fit_to_content: FitToContent::default(),

            epsilon: 10.0,
            canvas: CanvasConfig::default(),

            shortcuts,
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShapeConfig {
    #[cfg_attr(feature = "napi-support", napi(js_name = "line_color"))]
    pub line_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "fill_color"))]
    pub fill_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "vertex_fill_color"))]
    pub vertex_fill_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "select_line_color"))]
    pub select_line_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "select_fill_color"))]
    pub select_fill_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "hvertex_fill_color"))]
    pub hvertex_fill_color: Vec<i32>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "point_size"))]
    pub point_size: i32,
}

impl Default for ShapeConfig {
    fn default() -> Self {
        ShapeConfig {
            line_color: vec![0, 255, 0, 128],
            fill_color: vec![0, 0, 0, 64],
            vertex_fill_color: vec![0, 255, 0, 255],
            select_line_color: vec![255, 255, 255, 255],
            select_fill_color: vec![0, 255, 0, 64],
            hvertex_fill_color: vec![255, 255, 255, 255],
            point_size: 8,
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    #[cfg_attr(feature = "napi-support", napi(js_name = "default"))]
    pub default: String,
}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            default: "Sam2 (balanced)".to_string(),
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DockConfig {
    #[cfg_attr(feature = "napi-support", napi(js_name = "show"))]
    pub show: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "closable"))]
    pub closable: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "movable"))]
    pub movable: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "floatable"))]
    pub floatable: bool,
}

impl Default for DockConfig {
    fn default() -> Self {
        DockConfig {
            show: true,
            closable: true,
            movable: true,
            floatable: true,
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FitToContent {
    #[cfg_attr(feature = "napi-support", napi(js_name = "column"))]
    pub column: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "row"))]
    pub row: bool,
}

impl Default for FitToContent {
    fn default() -> Self {
        FitToContent {
            column: true,
            row: false,
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Crosshair {
    #[cfg_attr(feature = "napi-support", napi(js_name = "polygon"))]
    pub polygon: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "rectangle"))]
    pub rectangle: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "circle"))]
    pub circle: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "line"))]
    pub line: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "point"))]
    pub point: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "linestrip"))]
    pub linestrip: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "ai_polygon"))]
    pub ai_polygon: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "ai_mask"))]
    pub ai_mask: bool,
}

impl Default for Crosshair {
    fn default() -> Self {
        Crosshair {
            polygon: false,
            rectangle: true,
            circle: false,
            line: false,
            point: false,
            linestrip: false,
            ai_polygon: false,
            ai_mask: false,
        }
    }
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanvasConfig {
    #[cfg_attr(feature = "napi-support", napi(js_name = "fill_drawing"))]
    pub fill_drawing: bool,
    #[cfg_attr(feature = "napi-support", napi(js_name = "double_click"))]
    pub double_click: Option<String>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "num_backups"))]
    pub num_backups: i32,
    #[cfg_attr(feature = "napi-support", napi(js_name = "crosshair"))]
    pub crosshair: Crosshair,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        CanvasConfig {
            fill_drawing: true,
            double_click: Some("close".to_string()),
            num_backups: 10,
            crosshair: Crosshair::default(),
        }
    }
}

#[cfg_attr(feature = "napi-support", napi)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Shortcut {
    Single(String),
    Multiple(Vec<String>),
}

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Shortcuts {
    #[cfg_attr(feature = "napi-support", napi(js_name = "close"))]
    pub close: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "open"))]
    pub open: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "open_dir"))]
    pub open_dir: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "quit"))]
    pub quit: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "save"))]
    pub save: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "save_as"))]
    pub save_as: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "save_to"))]
    pub save_to: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "delete_file"))]
    pub delete_file: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "delete_image_file"))]
    pub delete_image_file: Option<Shortcut>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "open_next"))]
    pub open_next: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "open_prev"))]
    pub open_prev: Option<Shortcut>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "zoom_in"))]
    pub zoom_in: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "zoom_out"))]
    pub zoom_out: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "zoom_to_original"))]
    pub zoom_to_original: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "fit_window"))]
    pub fit_window: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "fit_width"))]
    pub fit_width: Option<Shortcut>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "create_polygon"))]
    pub create_polygon: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "create_rectangle"))]
    pub create_rectangle: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "create_circle"))]
    pub create_circle: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "create_line"))]
    pub create_line: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "create_point"))]
    pub create_point: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "create_linestrip"))]
    pub create_linestrip: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "edit_shape"))]
    pub edit_shape: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "delete_shape"))]
    pub delete_shape: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "duplicate_shape"))]
    pub duplicate_shape: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "copy_shape"))]
    pub copy_shape: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "paste_shape"))]
    pub paste_shape: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "undo"))]
    pub undo: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "undo_last_point"))]
    pub undo_last_point: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "edit_label"))]
    pub edit_label: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "toggle_keep_prev_mode"))]
    pub toggle_keep_prev_mode: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "remove_selected_point"))]
    pub remove_selected_point: Option<Shortcut>,

    #[cfg_attr(feature = "napi-support", napi(js_name = "show_all_shapes"))]
    pub show_all_shapes: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "hide_all_shapes"))]
    pub hide_all_shapes: Option<Shortcut>,
    #[cfg_attr(feature = "napi-support", napi(js_name = "toggle_all_shapes"))]
    pub toggle_all_shapes: Option<Shortcut>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Shortcuts {
            close: Some(Shortcut::Single("Ctrl+W".to_string())),
            open: Some(Shortcut::Single("Ctrl+O".to_string())),
            open_dir: Some(Shortcut::Single("Ctrl+U".to_string())),
            quit: Some(Shortcut::Single("Ctrl+Q".to_string())),
            save: Some(Shortcut::Single("Ctrl+S".to_string())),
            save_as: Some(Shortcut::Single("Ctrl+Shift+S".to_string())),
            save_to: None,
            delete_file: Some(Shortcut::Single("Ctrl+Delete".to_string())),
            delete_image_file: Some(Shortcut::Single("Ctrl+Shift+Delete".to_string())),

            open_next: Some(Shortcut::Multiple(vec![
                "D".to_string(),
                "Ctrl+Shift+D".to_string(),
            ])),
            open_prev: Some(Shortcut::Multiple(vec![
                "A".to_string(),
                "Ctrl+Shift+A".to_string(),
            ])),

            zoom_in: Some(Shortcut::Single("Ctrl++".to_string())),
            zoom_out: Some(Shortcut::Single("Ctrl+-".to_string())),
            zoom_to_original: Some(Shortcut::Single("Ctrl+0".to_string())),
            fit_window: Some(Shortcut::Single("Ctrl+F".to_string())),
            fit_width: Some(Shortcut::Single("Ctrl+Shift+F".to_string())),

            create_polygon: Some(Shortcut::Single("Ctrl+N".to_string())),
            create_rectangle: Some(Shortcut::Single("Ctrl+R".to_string())),
            create_circle: None,
            create_line: None,
            create_point: None,
            create_linestrip: None,
            edit_shape: Some(Shortcut::Single("Ctrl+J".to_string())),
            delete_shape: Some(Shortcut::Single("Delete".to_string())),
            duplicate_shape: Some(Shortcut::Single("Ctrl+D".to_string())),
            copy_shape: Some(Shortcut::Single("Ctrl+C".to_string())),
            paste_shape: Some(Shortcut::Single("Ctrl+V".to_string())),
            undo: Some(Shortcut::Single("Ctrl+Z".to_string())),
            undo_last_point: Some(Shortcut::Single("Ctrl+Z".to_string())),
            edit_label: Some(Shortcut::Single("Ctrl+E".to_string())),
            toggle_keep_prev_mode: Some(Shortcut::Single("Ctrl+P".to_string())),
            remove_selected_point: Some(Shortcut::Single("Backspace".to_string())),

            show_all_shapes: None,
            hide_all_shapes: None,
            toggle_all_shapes: Some(Shortcut::Single("T".to_string())),
        }
    }
}
