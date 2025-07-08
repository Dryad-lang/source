// src/oak/resolver.rs
//! Sistema de resolução de dependências

use std::collections::{HashMap, HashSet, VecDeque};
use semver::{Version, VersionReq};

use super::config::{OakConfig, OakDependency, OakPackage};

/// Erro de resolução de dependências
#[derive(Debug, Clone)]
pub enum ResolverError {
    CircularDependency(Vec<String>),
    VersionConflict(String, Vec<String>),
    PackageNotFound(String),
    InvalidVersion(String, String),
}

/// Grafo de dependências
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

/// Nó no grafo de dependências
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub name: String,
    pub version: String,
    pub package: OakPackage,
    pub dev: bool,
    pub optional: bool,
}

/// Aresta no grafo de dependências
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub constraint: String,
}

/// Resolução de dependências
pub struct DependencyResolver {
    cache: HashMap<String, Vec<OakPackage>>,
    strict_mode: bool,
}

/// Resolvedor de versões
pub struct VersionResolver {
    strategy: VersionStrategy,
}

/// Estratégias de resolução de versão
#[derive(Debug, Clone)]
pub enum VersionStrategy {
    Exact,      // Versão exata
    Latest,     // Sempre a mais recente
    Semver,     // Respeitar semantic versioning
    Conservative, // Mudanças mínimas
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverError::CircularDependency(cycle) => {
                write!(f, "Circular dependency detected: {}", cycle.join(" -> "))
            }
            ResolverError::VersionConflict(package, versions) => {
                write!(f, "Version conflict for {}: {}", package, versions.join(", "))
            }
            ResolverError::PackageNotFound(package) => {
                write!(f, "Package not found: {}", package)
            }
            ResolverError::InvalidVersion(package, version) => {
                write!(f, "Invalid version {} for package {}", version, package)
            }
        }
    }
}

impl std::error::Error for ResolverError {}

impl DependencyResolver {
    /// Cria um novo resolvedor
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            strict_mode: false,
        }
    }

    /// Cria um resolvedor em modo estrito
    pub fn strict() -> Self {
        Self {
            cache: HashMap::new(),
            strict_mode: true,
        }
    }

    /// Resolve dependências de um projeto
    pub fn resolve(&self, config: &OakConfig) -> Result<HashMap<String, OakPackage>, ResolverError> {
        let mut resolved = HashMap::new();
        let mut graph = DependencyGraph::new();
        
        // Adicionar dependências de produção
        for (name, dep) in &config.dependencies {
            self.resolve_dependency(name, dep, &mut graph, &mut resolved, false)?;
        }
        
        // Adicionar dependências de desenvolvimento se não em modo estrito
        if !self.strict_mode {
            for (name, dep) in &config.dev_dependencies {
                self.resolve_dependency(name, dep, &mut graph, &mut resolved, true)?;
            }
        }
        
        // Verificar dependências circulares
        self.check_circular_dependencies(&graph)?;
        
        // Resolver conflitos de versão
        self.resolve_version_conflicts(&mut resolved)?;
        
        Ok(resolved)
    }

    /// Resolve uma dependência específica
    fn resolve_dependency(
        &self,
        name: &str,
        dependency: &OakDependency,
        graph: &mut DependencyGraph,
        resolved: &mut HashMap<String, OakPackage>,
        is_dev: bool,
    ) -> Result<(), ResolverError> {
        
        // Se já resolvido, verificar compatibilidade de versão
        if let Some(existing) = resolved.get(name) {
            if !self.is_version_compatible(&existing.version, &dependency.version) {
                return Err(ResolverError::VersionConflict(
                    name.to_string(),
                    vec![existing.version.clone(), dependency.version.clone()],
                ));
            }
            return Ok(());
        }

        // Buscar pacote no cache/registry
        let package = self.find_best_package(name, &dependency.version)?;
        
        // Adicionar ao grafo
        let node = DependencyNode {
            name: name.to_string(),
            version: package.version.clone(),
            package: package.clone(),
            dev: is_dev,
            optional: dependency.optional,
        };
        graph.add_node(node);
        
        // Resolver dependências transitivas
        for (dep_name, dep_dependency) in &package.dependencies {
            // Adicionar aresta no grafo
            graph.add_edge(DependencyEdge {
                from: name.to_string(),
                to: dep_name.clone(),
                constraint: dep_dependency.version.clone(),
            });
            
            // Resolver recursivamente
            self.resolve_dependency(dep_name, dep_dependency, graph, resolved, is_dev)?;
        }
        
        resolved.insert(name.to_string(), package);
        Ok(())
    }

    /// Busca o melhor pacote para uma dependência
    fn find_best_package(&self, name: &str, version: &str) -> Result<OakPackage, ResolverError> {
        // Verificar cache primeiro
        if let Some(packages) = self.cache.get(name) {
            for package in packages {
                if self.is_version_compatible(&package.version, version) {
                    return Ok(package.clone());
                }
            }
        }
        
        // Se não encontrado, simular busca (normalmente seria no registry)
        // Por enquanto, criar um pacote mock
        if version == "latest" || version.starts_with("^") || version.starts_with("~") {
            Ok(OakPackage::new(name.to_string(), "1.0.0".to_string()))
        } else {
            Ok(OakPackage::new(name.to_string(), version.to_string()))
        }
    }

    /// Verifica se duas versões são compatíveis
    fn is_version_compatible(&self, installed: &str, required: &str) -> bool {
        if required == "latest" {
            return true;
        }
        
        // Tentar parsing semver
        if let (Ok(installed_ver), Ok(required_req)) = (
            Version::parse(installed),
            VersionReq::parse(required)
        ) {
            return required_req.matches(&installed_ver);
        }
        
        // Fallback para comparação exata
        installed == required
    }

    /// Verifica dependências circulares
    fn check_circular_dependencies(&self, graph: &DependencyGraph) -> Result<(), ResolverError> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node_name in graph.nodes.keys() {
            if !visited.contains(node_name) {
                if let Some(cycle) = self.dfs_cycle_check(
                    node_name, 
                    graph, 
                    &mut visited, 
                    &mut rec_stack, 
                    &mut Vec::new()
                ) {
                    return Err(ResolverError::CircularDependency(cycle));
                }
            }
        }
        
        Ok(())
    }

    /// DFS para detectar ciclos
    fn dfs_cycle_check(
        &self,
        node: &str,
        graph: &DependencyGraph,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        // Encontrar dependências deste nó
        for edge in &graph.edges {
            if edge.from == node {
                let dep = &edge.to;
                
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle_check(dep, graph, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Ciclo encontrado
                    let cycle_start = path.iter().position(|x| x == dep).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(dep.to_string());
                    return Some(cycle);
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
        None
    }

    /// Resolve conflitos de versão
    fn resolve_version_conflicts(&self, resolved: &mut HashMap<String, OakPackage>) -> Result<(), ResolverError> {
        // Por enquanto, apenas verificar se há conflitos óbvios
        // Em uma implementação completa, tentaria encontrar versões compatíveis
        
        for (name, package) in resolved.iter() {
            // Verificar se a versão é válida
            if package.version.is_empty() {
                return Err(ResolverError::InvalidVersion(
                    name.clone(), 
                    package.version.clone()
                ));
            }
        }
        
        Ok(())
    }

    /// Adiciona pacotes ao cache
    pub fn add_to_cache(&mut self, packages: Vec<OakPackage>) {
        for package in packages {
            self.cache
                .entry(package.name.clone())
                .or_insert_with(Vec::new)
                .push(package);
        }
    }

    /// Limpa o cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Cria um novo grafo de dependências
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Adiciona um nó ao grafo
    pub fn add_node(&mut self, node: DependencyNode) {
        self.nodes.insert(node.name.clone(), node);
    }

    /// Adiciona uma aresta ao grafo
    pub fn add_edge(&mut self, edge: DependencyEdge) {
        self.edges.push(edge);
    }

    /// Obtém dependências diretas de um nó
    pub fn get_dependencies(&self, node_name: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node_name)
            .map(|edge| edge.to.clone())
            .collect()
    }

    /// Obtém todos os nós em ordem topológica
    pub fn topological_sort(&self) -> Result<Vec<String>, ResolverError> {
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Calcular in-degree de cada nó
        for node_name in self.nodes.keys() {
            in_degree.insert(node_name.clone(), 0);
        }

        for edge in &self.edges {
            *in_degree.get_mut(&edge.to).unwrap() += 1;
        }

        // Adicionar nós sem dependências à fila
        for (node_name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_name.clone());
            }
        }

        // Processar nós
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            // Reduzir in-degree dos vizinhos
            for edge in &self.edges {
                if edge.from == node {
                    let neighbor = &edge.to;
                    *in_degree.get_mut(neighbor).unwrap() -= 1;
                    
                    if in_degree[neighbor] == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        // Verificar se há ciclos
        if result.len() != self.nodes.len() {
            Err(ResolverError::CircularDependency(
                self.nodes.keys().cloned().collect()
            ))
        } else {
            Ok(result)
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionResolver {
    /// Cria um novo resolvedor de versões
    pub fn new(strategy: VersionStrategy) -> Self {
        Self { strategy }
    }

    /// Resolve a melhor versão baseada na estratégia
    pub fn resolve_best_version(&self, available: &[String], constraint: &str) -> Option<String> {
        match self.strategy {
            VersionStrategy::Exact => {
                if available.contains(&constraint.to_string()) {
                    Some(constraint.to_string())
                } else {
                    None
                }
            }
            VersionStrategy::Latest => {
                available.last().cloned()
            }
            VersionStrategy::Semver => {
                self.resolve_semver(available, constraint)
            }
            VersionStrategy::Conservative => {
                self.resolve_conservative(available, constraint)
            }
        }
    }

    /// Resolve usando semantic versioning
    fn resolve_semver(&self, available: &[String], constraint: &str) -> Option<String> {
        let req = VersionReq::parse(constraint).ok()?;
        
        for version_str in available.iter().rev() {
            if let Ok(version) = Version::parse(version_str) {
                if req.matches(&version) {
                    return Some(version_str.clone());
                }
            }
        }
        
        None
    }

    /// Resolve de forma conservadora
    fn resolve_conservative(&self, available: &[String], constraint: &str) -> Option<String> {
        // Tentar encontrar a versão mais próxima sem quebrar compatibilidade
        if constraint == "latest" {
            return available.last().cloned();
        }
        
        // Se for um constraint específico, usar semver
        self.resolve_semver(available, constraint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_resolver() {
        let resolver = DependencyResolver::new();
        let config = OakConfig::default();
        
        let result = resolver.resolve(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_compatibility() {
        let resolver = DependencyResolver::new();
        
        assert!(resolver.is_version_compatible("1.0.0", "^1.0.0"));
        assert!(resolver.is_version_compatible("1.2.3", "~1.2.0"));
        assert!(!resolver.is_version_compatible("2.0.0", "^1.0.0"));
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        let node = DependencyNode {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            package: OakPackage::new("test".to_string(), "1.0.0".to_string()),
            dev: false,
            optional: false,
        };
        
        graph.add_node(node);
        assert!(graph.nodes.contains_key("test"));
    }

    #[test]
    fn test_version_resolver() {
        let resolver = VersionResolver::new(VersionStrategy::Latest);
        let available = vec!["1.0.0".to_string(), "1.1.0".to_string(), "2.0.0".to_string()];
        
        let result = resolver.resolve_best_version(&available, "latest");
        assert_eq!(result, Some("2.0.0".to_string()));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let resolver = DependencyResolver::new();
        let mut graph = DependencyGraph::new();
        
        // Criar dependência circular: A -> B -> A
        graph.add_node(DependencyNode {
            name: "A".to_string(),
            version: "1.0.0".to_string(),
            package: OakPackage::new("A".to_string(), "1.0.0".to_string()),
            dev: false,
            optional: false,
        });
        
        graph.add_node(DependencyNode {
            name: "B".to_string(),
            version: "1.0.0".to_string(),
            package: OakPackage::new("B".to_string(), "1.0.0".to_string()),
            dev: false,
            optional: false,
        });
        
        graph.add_edge(DependencyEdge {
            from: "A".to_string(),
            to: "B".to_string(),
            constraint: "1.0.0".to_string(),
        });
        
        graph.add_edge(DependencyEdge {
            from: "B".to_string(),
            to: "A".to_string(),
            constraint: "1.0.0".to_string(),
        });
        
        let result = resolver.check_circular_dependencies(&graph);
        assert!(result.is_err());
    }
}
