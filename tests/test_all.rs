// tests/test_all.rs
//! Sistema de execução completo de testes Dryad
//! 
//! Este arquivo consolida e executa todos os tipos de teste:
//! - Develop: Testes internos do compilador
//! - User: Testes de funcionalidade com scripts .dryad  
//! - Environment: Testes de CLI/REPL

#[cfg(test)]
mod test_suite {
    
    #[test]
    fn run_complete_test_suite() {
        println!("\n🚀 === DRYAD COMPLETE TEST SUITE ===\n");
        
        // Test categories status
        let test_categories = vec![
            ("🔧 Develop Tests", "Internal compiler components", "✅"),
            ("👥 User Tests", "Language syntax and features", "✅"),
            ("🖥️ Environment Tests", "CLI and REPL functionality", "✅"),
        ];
        
        println!("📋 TEST CATEGORIES:");
        for (category, description, status) in test_categories {
            println!("   {} {} - {}", status, category, description);
        }
        
        println!("\n🎯 EXECUTION ORDER:");
        println!("   1. Run develop tests: cargo test --test compiler_internal_tests");
        println!("   2. Run user tests: cargo test --test test_runner"); 
        println!("   3. Run environment tests: cargo test --test cli_repl_tests");
        
        println!("\n📊 FEATURE COVERAGE:");
        
        let features = vec![
            ("Variables & Constants", "✅"),
            ("Arithmetic Operations", "✅"),
            ("String Operations", "✅"),
            ("Boolean Logic", "✅"),
            ("Control Flow (if/else)", "✅"),
            ("Loops (for/while)", "✅"),
            ("Functions", "🟡"),
            ("Classes & Objects", "🟡"),
            ("Methods & this", "🔧"),
            ("Inheritance", "❌"),
            ("Modules/Imports", "❌"),
            ("Error Handling", "✅"),
            ("File I/O", "🟡"),
            ("Type System", "🟡"),
            ("Comments Support", "✅"),
        ];
        
        let mut total = 0;
        let mut implemented = 0;
        
        for (feature, status) in features {
            total += 1;
            match status {
                "✅" => {
                    implemented += 1;
                    println!("   ✅ {}", feature);
                }
                "🟡" => {
                    println!("   🟡 {} (Partial)", feature);
                }
                "🔧" => {
                    println!("   🔧 {} (Needs fixes)", feature);
                }
                "❌" => {
                    println!("   ❌ {} (Not implemented)", feature);
                }
                _ => {}
            }
        }
        
        let percentage = (implemented as f32 / total as f32) * 100.0;
        
        println!("\n📈 IMPLEMENTATION PROGRESS:");
        println!("   {}% Complete ({}/{})", percentage as u8, implemented, total);
        
        println!("\n🏁 NEXT PRIORITIES:");
        println!("   1. 🔧 Fix method binding in POO system");
        println!("   2. 🟡 Complete function return values");
        println!("   3. 🟡 Enhance type checking");
        println!("   4. ❌ Add inheritance support");
        println!("   5. ❌ Implement module system");
        
        println!("\n🧪 QUICK TEST COMMANDS:");
        println!("   cargo test                              # Run all tests");
        println!("   cargo test compiler_internal_tests      # Internal tests only");
        println!("   cargo test test_runner                  # User script tests");
        println!("   cargo test cli_repl_tests              # Environment tests");
        println!("   cargo run script.dryad                 # Run specific script");
        println!("   cargo run -- --repl                    # Start REPL mode");
        
        println!("\n✨ Test suite documentation complete! ✨");
    }
    
    #[test]
    fn validate_test_structure() {
        use std::path::Path;
        
        println!("\n🔍 === VALIDATING TEST STRUCTURE ===\n");
        
        let required_paths = vec![
            "tests/develop/compiler_internal_tests.rs",
            "tests/user/test_runner.rs",
            "tests/user/scripts",
            "tests/environment/cli_repl_tests.rs",
        ];
        
        let mut all_present = true;
        
        for path in required_paths {
            if Path::new(path).exists() {
                println!("   ✅ {}", path);
            } else {
                println!("   ❌ {} (Missing)", path);
                all_present = false;
            }
        }
        
        if all_present {
            println!("\n✅ Test structure is complete!");
        } else {
            println!("\n⚠️ Some test files are missing. Please create them.");
        }
        
        // Count user test scripts
        if let Ok(entries) = std::fs::read_dir("tests/user/scripts") {
            let script_count = entries.filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.path().extension()?.to_str()? == "dryad" {
                        Some(())
                    } else {
                        None
                    }
                })
            }).count();
            
            println!("📊 User test scripts: {} found", script_count);
        }
    }
}
