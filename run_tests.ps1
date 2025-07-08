// run_all_tests.ps1
# Script para executar todos os testes atualizados

$tests = @(
    "tests\user\scripts\01_variables.dryad",
    "tests\user\scripts\16_namespace_basic.dryad",
    "tests\user\scripts\18_using_basic.dryad",
    "tests\user\scripts\19_using_alias.dryad",
    "tests\user\scripts\20_using_multiple.dryad",
    "tests\user\scripts\26_io_console_static.dryad",
    "tests\user\scripts\27_io_filesystem_static.dryad",
    "tests\user\scripts\28_core_types_static.dryad",
    "tests\user\scripts\29_system_modules_static.dryad",
    "tests\user\scripts\30_multiple_imports_static.dryad",
    "tests\user\scripts\31_static_vs_instance.dryad",
    "tests\user\scripts\32_error_handling_static.dryad",
    "tests\user\scripts\33_organized_imports.dryad",
    "tests\user\scripts\34_best_practices.dryad"
)

foreach ($test in $tests) {
    Write-Host "=== Running $test ===" -ForegroundColor Green
    .\target\debug\dryad.exe $test
    Write-Host ""
}
