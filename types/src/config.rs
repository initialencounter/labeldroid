use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi-support", napi(object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub auto_save: bool,
    pub display_label_popup: bool,
    pub with_image_data: bool,
    pub keep_prev: bool,
    pub keep_prev_scale: bool,
    pub keep_prev_brightness_contrast: bool,
    pub logger_level: String,

    pub flags: Option<String>,
    pub label_flags: Option<String>,
    pub labels: Option<Vec<String>>,
    pub file_search: Option<String>,
    pub sort_labels: bool,
    pub validate_label: Option<String>,

    pub default_shape_color: Vec<i32>,
    pub shape_color: Option<String>,
    pub shift_auto_shape_color: i32,
    pub label_colors: Option<String>,

    pub shape: ShapeConfig,
    pub ai: AiConfig,

    pub flag_dock: DockConfig,
    pub label_dock: DockConfig,
    pub shape_dock: DockConfig,
    pub file_dock: DockConfig,

    pub show_label_text_field: bool,
    pub label_completion: String,
    pub fit_to_content: FitToContent,

    pub epsilon: f64,
    pub canvas: CanvasConfig,

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShapeConfig {
    pub line_color: Vec<i32>,
    pub fill_color: Vec<i32>,
    pub vertex_fill_color: Vec<i32>,
    pub select_line_color: Vec<i32>,
    pub select_fill_color: Vec<i32>,
    pub hvertex_fill_color: Vec<i32>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    pub default: String,
}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            default: "Sam2 (balanced)".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DockConfig {
    pub show: bool,
    pub closable: bool,
    pub movable: bool,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FitToContent {
    pub column: bool,
    pub row: bool,
}

impl Default for FitToContent {
    fn default() -> Self {
        FitToContent { column: true, row: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Crosshair {
    pub polygon: bool,
    pub rectangle: bool,
    pub circle: bool,
    pub line: bool,
    pub point: bool,
    pub linestrip: bool,
    pub ai_polygon: bool,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanvasConfig {
    pub fill_drawing: bool,
    pub double_click: Option<String>,
    pub num_backups: i32,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Shortcuts {
    pub close: Option<String>,
    pub open: Option<String>,
    pub open_dir: Option<String>,
    pub quit: Option<String>,
    pub save: Option<String>,
    pub save_as: Option<String>,
    pub save_to: Option<String>,
    pub delete_file: Option<String>,
    pub delete_image_file: Option<String>,

    pub open_next: Option<String>,
    pub open_prev: Option<String>,

    pub zoom_in: Option<String>,
    pub zoom_out: Option<String>,
    pub zoom_to_original: Option<String>,
    pub fit_window: Option<String>,
    pub fit_width: Option<String>,

    pub create_polygon: Option<String>,
    pub create_rectangle: Option<String>,
    pub create_circle: Option<String>,
    pub create_line: Option<String>,
    pub create_point: Option<String>,
    pub create_linestrip: Option<String>,
    pub edit_shape: Option<String>,
    pub delete_shape: Option<String>,
    pub duplicate_shape: Option<String>,
    pub copy_shape: Option<String>,
    pub paste_shape: Option<String>,
    pub undo: Option<String>,
    pub undo_last_point: Option<String>,
    pub edit_label: Option<String>,
    pub toggle_keep_prev_mode: Option<String>,
    pub remove_selected_point: Option<String>,

    pub show_all_shapes: Option<String>,
    pub hide_all_shapes: Option<String>,
    pub toggle_all_shapes: Option<String>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Shortcuts {
            close: Some("Ctrl+W".to_string()),
            open: Some("Ctrl+O".to_string()),
            open_dir: Some("Ctrl+U".to_string()),
            quit: Some("Ctrl+Q".to_string()),
            save: Some("Ctrl+S".to_string()),
            save_as: Some("Ctrl+Shift+S".to_string()),
            save_to: None,
            delete_file: Some("Ctrl+Delete".to_string()),
            delete_image_file: Some("Ctrl+Shift+Delete".to_string()),

            open_next: Some("D".to_string()),
            open_prev: Some("A".to_string()),

            zoom_in: Some("Ctrl++".to_string()),
            zoom_out: Some("Ctrl+-".to_string()),
            zoom_to_original: Some("Ctrl+0".to_string()),
            fit_window: Some("Ctrl+F".to_string()),
            fit_width: Some("Ctrl+Shift+F".to_string()),

            create_polygon: Some("Ctrl+N".to_string()),
            create_rectangle: Some("Ctrl+R".to_string()),
            create_circle: None,
            create_line: None,
            create_point: None,
            create_linestrip: None,
            edit_shape: Some("Ctrl+J".to_string()),
            delete_shape: Some("Delete".to_string()),
            duplicate_shape: Some("Ctrl+D".to_string()),
            copy_shape: Some("Ctrl+C".to_string()),
            paste_shape: Some("Ctrl+V".to_string()),
            undo: Some("Ctrl+Z".to_string()),
            undo_last_point: Some("Ctrl+Z".to_string()),
            edit_label: Some("Ctrl+E".to_string()),
            toggle_keep_prev_mode: Some("Ctrl+P".to_string()),
            remove_selected_point: Some("Backspace".to_string()),

            show_all_shapes: None,
            hide_all_shapes: None,
            toggle_all_shapes: Some("T".to_string()),
        }
    }
}
