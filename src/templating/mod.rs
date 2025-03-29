use crate::error::app_error::AppError;
use handlebars::Handlebars;
use std::sync::RwLock;
use std::sync::OnceLock;
use serde::Serialize;

// Global handlebars instance that gets initialized once and can be reused
static HANDLEBARS: OnceLock<RwLock<Handlebars>> = OnceLock::new();

/// Initialize the Handlebars registry and register all templates
pub fn init_templates() -> Result<(), AppError> {
    let mut handlebars = Handlebars::new();
    
    // Register all templates here
    handlebars
        .register_template_file("collection_view", "templates/collection/view.hbs")
        .map_err(|err| AppError::CustomError {
            status_code: 500,
            message: format!("Failed to load template file: {}", err),
        })?;
    
    // Add more templates as needed:
    // handlebars.register_template_file("another_template", "templates/another_template.hbs")?;

    // Store the initialized Handlebars instance
    HANDLEBARS.get_or_init(|| RwLock::new(handlebars));
    
    Ok(())
}

/// Generic function to render a template with the provided data
pub fn render<T: Serialize>(template_name: &str, data: &T) -> Result<String, AppError> {
    // Get the Handlebars instance, initializing if necessary
    let handlebars_instance = HANDLEBARS.get_or_init(|| {
        let mut handlebars = Handlebars::new();
        // Register basic templates in case init_templates wasn't called
        let _ = handlebars.register_template_file("collection_view", "templates/collection/view.hbs");
        RwLock::new(handlebars)
    });
    
    // Acquire read lock and render
    let handlebars = handlebars_instance.read()
        .map_err(|_| AppError::CustomError {
            status_code: 500,
            message: "Failed to acquire template engine lock".to_string(),
        })?;
    
    handlebars
        .render(template_name, data)
        .map_err(|err| AppError::CustomError {
            status_code: 500,
            message: format!("Template rendering error: {}", err),
        })
}

/// Register a new template at runtime
pub fn register_template(name: &str, template_path: &str) -> Result<(), AppError> {
    let handlebars_instance = HANDLEBARS.get_or_init(|| RwLock::new(Handlebars::new()));
    
    let mut handlebars = handlebars_instance.write()
        .map_err(|_| AppError::CustomError {
            status_code: 500,
            message: "Failed to acquire template engine lock for writing".to_string(),
        })?;
        
    handlebars
        .register_template_file(name, template_path)
        .map_err(|err| AppError::CustomError {
            status_code: 500,
            message: format!("Failed to register template '{}': {}", name, err),
        })
}