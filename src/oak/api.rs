// src/oak/api.rs
//! APIs externas para integração com outras ferramentas

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::path::PathBuf;
use serde_json::{Value, json};

use super::{OakManager, OakOptions};

/// API principal do Oak para integração externa
pub struct OakApi {
    manager: OakManager,
}

/// API C-style para integração com outras linguagens
pub struct ExternalApi;

/// Configuração para API externa
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub workspace_path: Option<PathBuf>,
    pub config_file: Option<String>,
    pub registry_url: Option<String>,
    pub verbose: bool,
    pub force: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            workspace_path: None,
            config_file: None,
            registry_url: None,
            verbose: false,
            force: false,
        }
    }
}

impl OakApi {
    /// Cria uma nova instância da API
    pub fn new() -> Self {
        Self {
            manager: OakManager::new(),
        }
    }

    /// Cria uma API com configurações customizadas
    pub fn with_config(config: ApiConfig) -> Self {
        let mut options = OakOptions::default();
        options.verbose = config.verbose;
        options.force = config.force;
        
        if let Some(config_file) = config.config_file {
            options.config_path = Some(PathBuf::from(config_file));
        }

        Self {
            manager: OakManager::with_options(options),
        }
    }

    /// Inicializa um projeto Oak
    pub fn init_project(&self, name: Option<String>, description: Option<String>) -> Value {
        let result = self.manager.init_project(name, description);
        result.to_json()
    }

    /// Adiciona uma dependência
    pub fn add_dependency(&self, name: String, version: Option<String>, dev: bool) -> Value {
        let result = self.manager.add_package(name, version, dev);
        result.to_json()
    }

    /// Remove uma dependência
    pub fn remove_dependency(&self, name: String) -> Value {
        let result = self.manager.remove_package(name);
        result.to_json()
    }

    /// Lista dependências
    pub fn list_dependencies(&self, include_dev: bool) -> Value {
        let result = self.manager.list_packages(include_dev, true);
        result.to_json()
    }

    /// Instala todas as dependências
    pub fn install_dependencies(&self) -> Value {
        let result = self.manager.install();
        result.to_json()
    }

    /// Obtém informações do projeto
    pub fn get_project_info(&self) -> Value {
        let result = self.manager.get_project_info();
        result.to_json()
    }

    /// Valida a configuração
    pub fn validate_config(&self) -> Value {
        let result = self.manager.validate();
        result.to_json()
    }

    /// Adiciona um caminho de biblioteca
    pub fn add_lib_path(&self, path: String) -> Value {
        let result = self.manager.add_lib_path(path);
        result.to_json()
    }

    /// Remove um caminho de biblioteca
    pub fn remove_lib_path(&self, path: String) -> Value {
        let result = self.manager.remove_lib_path(path);
        result.to_json()
    }

    /// Executa um script
    pub fn run_script(&self, script_name: String) -> Value {
        let result = self.manager.run_script(script_name);
        result.to_json()
    }

    /// Verifica se o projeto existe
    pub fn project_exists(&self) -> bool {
        self.manager.project_exists()
    }

    /// Converte resultado para JSON string
    pub fn to_json_string(&self, value: &Value) -> String {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
    }
}

impl Default for OakApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementação da API C para integração externa
impl ExternalApi {
    /// Converte string C para Rust String
    unsafe fn c_str_to_string(c_str: *const c_char) -> Option<String> {
        if c_str.is_null() {
            None
        } else {
            CStr::from_ptr(c_str).to_str().ok().map(|s| s.to_string())
        }
    }

    /// Converte Rust String para C string
    fn string_to_c_str(s: String) -> *mut c_char {
        CString::new(s).unwrap_or_else(|_| CString::new("").unwrap()).into_raw()
    }

    /// Converte Value para JSON C string
    fn value_to_c_json(value: Value) -> *mut c_char {
        let json_str = serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string());
        Self::string_to_c_str(json_str)
    }
}

/// API C para integração externa
#[no_mangle]
pub extern "C" fn oak_api_create() -> *mut OakApi {
    Box::into_raw(Box::new(OakApi::new()))
}

/// Destrói uma instância da API
#[no_mangle]
pub extern "C" fn oak_api_destroy(api: *mut OakApi) {
    if !api.is_null() {
        unsafe {
            let _ = Box::from_raw(api);
        }
    }
}

/// Inicializa um projeto
#[no_mangle]
pub extern "C" fn oak_api_init_project(
    api: *mut OakApi,
    name: *const c_char,
    description: *const c_char
) -> *mut c_char {
    if api.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid API instance\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let name_opt = ExternalApi::c_str_to_string(name);
        let desc_opt = ExternalApi::c_str_to_string(description);
        
        let result = api.init_project(name_opt, desc_opt);
        ExternalApi::value_to_c_json(result)
    }
}

/// Adiciona uma dependência
#[no_mangle]
pub extern "C" fn oak_api_add_dependency(
    api: *mut OakApi,
    name: *const c_char,
    version: *const c_char,
    is_dev: bool
) -> *mut c_char {
    if api.is_null() || name.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid parameters\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let name_str = ExternalApi::c_str_to_string(name).unwrap_or_default();
        let version_opt = ExternalApi::c_str_to_string(version);
        
        let result = api.add_dependency(name_str, version_opt, is_dev);
        ExternalApi::value_to_c_json(result)
    }
}

/// Remove uma dependência
#[no_mangle]
pub extern "C" fn oak_api_remove_dependency(
    api: *mut OakApi,
    name: *const c_char
) -> *mut c_char {
    if api.is_null() || name.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid parameters\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let name_str = ExternalApi::c_str_to_string(name).unwrap_or_default();
        
        let result = api.remove_dependency(name_str);
        ExternalApi::value_to_c_json(result)
    }
}

/// Lista dependências
#[no_mangle]
pub extern "C" fn oak_api_list_dependencies(
    api: *mut OakApi,
    include_dev: bool
) -> *mut c_char {
    if api.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid API instance\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let result = api.list_dependencies(include_dev);
        ExternalApi::value_to_c_json(result)
    }
}

/// Instala dependências
#[no_mangle]
pub extern "C" fn oak_api_install_dependencies(api: *mut OakApi) -> *mut c_char {
    if api.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid API instance\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let result = api.install_dependencies();
        ExternalApi::value_to_c_json(result)
    }
}

/// Obtém informações do projeto
#[no_mangle]
pub extern "C" fn oak_api_get_project_info(api: *mut OakApi) -> *mut c_char {
    if api.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid API instance\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let result = api.get_project_info();
        ExternalApi::value_to_c_json(result)
    }
}

/// Valida configuração
#[no_mangle]
pub extern "C" fn oak_api_validate_config(api: *mut OakApi) -> *mut c_char {
    if api.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid API instance\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let result = api.validate_config();
        ExternalApi::value_to_c_json(result)
    }
}

/// Verifica se projeto existe
#[no_mangle]
pub extern "C" fn oak_api_project_exists(api: *mut OakApi) -> bool {
    if api.is_null() {
        return false;
    }

    unsafe {
        let api = &*api;
        api.project_exists()
    }
}

/// Adiciona um caminho de biblioteca
#[no_mangle]
pub extern "C" fn oak_api_add_lib_path(
    api: *mut OakApi,
    path: *const c_char
) -> *mut c_char {
    if api.is_null() || path.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid parameters\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let path_str = ExternalApi::c_str_to_string(path).unwrap_or_default();
        
        let result = api.add_lib_path(path_str);
        ExternalApi::value_to_c_json(result)
    }
}

/// Remove um caminho de biblioteca
#[no_mangle]
pub extern "C" fn oak_api_remove_lib_path(
    api: *mut OakApi,
    path: *const c_char
) -> *mut c_char {
    if api.is_null() || path.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid parameters\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let path_str = ExternalApi::c_str_to_string(path).unwrap_or_default();
        
        let result = api.remove_lib_path(path_str);
        ExternalApi::value_to_c_json(result)
    }
}

/// Executa um script
#[no_mangle]
pub extern "C" fn oak_api_run_script(
    api: *mut OakApi,
    script_name: *const c_char
) -> *mut c_char {
    if api.is_null() || script_name.is_null() {
        return ExternalApi::string_to_c_str("{\"success\": false, \"message\": \"Invalid parameters\"}".to_string());
    }

    unsafe {
        let api = &*api;
        let script_str = ExternalApi::c_str_to_string(script_name).unwrap_or_default();
        
        let result = api.run_script(script_str);
        ExternalApi::value_to_c_json(result)
    }
}

/// Libera memória de string C
#[no_mangle]
pub extern "C" fn oak_api_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Obtém versão do Oak
#[no_mangle]
pub extern "C" fn oak_api_get_version() -> *mut c_char {
    ExternalApi::string_to_c_str(crate::oak::OAK_VERSION.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[test]
    fn test_oak_api_creation() {
        let api = OakApi::new();
        assert!(!api.project_exists()); // Assumindo que não há projeto no diretório de teste
    }

    #[test]
    fn test_api_with_config() {
        let config = ApiConfig {
            verbose: true,
            force: false,
            ..Default::default()
        };
        
        let api = OakApi::with_config(config);
        assert!(!api.project_exists());
    }

    #[test]
    fn test_project_lifecycle() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let api = OakApi::new();
        
        // Inicializar projeto
        let result = api.init_project(Some("test-api".to_string()), None);
        assert!(result["success"].as_bool().unwrap_or(false));
        assert!(api.project_exists());
        
        // Adicionar dependência
        let result = api.add_dependency("test-dep".to_string(), Some("1.0.0".to_string()), false);
        // Note: pode falhar porque não há registry real, mas deve retornar JSON válido
        assert!(result.is_object());
        
        // Listar dependências
        let result = api.list_dependencies(true);
        assert!(result.is_object());
        
        // Validar configuração
        let result = api.validate_config();
        assert!(result.is_object());
        
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_c_api_functions() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        env::set_current_dir(temp_dir.path()).unwrap();
        
        // Criar API
        let api = oak_api_create();
        assert!(!api.is_null());
        
        // Verificar se projeto existe
        assert!(!oak_api_project_exists(api));
        
        // Inicializar projeto
        let name = CString::new("test-c-api").unwrap();
        let desc = CString::new("Test C API").unwrap();
        let result_ptr = oak_api_init_project(api, name.as_ptr(), desc.as_ptr());
        assert!(!result_ptr.is_null());
        
        // Liberar string resultado
        oak_api_free_string(result_ptr);
        
        // Verificar se projeto foi criado
        assert!(oak_api_project_exists(api));
        
        // Destruir API
        oak_api_destroy(api);
        
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_json_conversion() {
        let api = OakApi::new();
        let value = json!({"test": "value"});
        let json_string = api.to_json_string(&value);
        assert!(json_string.contains("test"));
        assert!(json_string.contains("value"));
    }
}
