

# $s = (
#   'rust_intro/d1_syntax',
#   'rust_intro/d1_syntax_generics',
#   'rust_intro/d1_syntax_loops',
#   'rust_intro/d1_syntax_result',
#   'rust_intro/d2_lifetimes_array',
#   'rust_intro/d2_lifetimes_box',
#   'rust_intro/d2_lifetimes_matter',
#   'rust_intro/d2_lifetimes_passing',
#   'rust_intro/d2_lifetimes_refcounter',
#   'rust_intro/d2_lifetimes_str',
#   'rust_intro/d3_tricks_errors',
#   'rust_intro/d3_tricks_mod',
#   'rust_intro/d3_tricks_newtype',
#   'rust_intro/d3_tricks_poly',
#   'rust_intro/d3_tricks_trait',
#   'rust_intro/v1_hello'
# )

Push-Location $PSScriptRoot

try {
  $rootFolder = Split-Path $PSScriptRoot -Parent
  $projectFolders = Get-ChildItem -Directory
  $projectFolders | ForEach-Object {
    if (-not (Test-Path "$($_.FullName)\Cargo.toml")) { continue; }
    $projectFolder = $_.FullName.Replace("$rootFolder\", '').Replace('\', '/')
    Write-Output $projectFolder
    $filename = $projectFolder.Replace('/', '_')
    @"
name: $projectFolder

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

defaults:
  run:
    working-directory: ./$projectFolder

jobs:
  build-window:
    runs-on: windows-latest
    steps:
    - uses: actions/checkout@v2
    - name: Run tests
      run: cargo test --verbose  
    - name: Build
      run: cargo build --verbose
    - name: Build Release
      run: cargo build --release --verbose
  build-linux:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Run tests
      run: cargo test --verbose  
    - name: Build
      run: cargo build --verbose
    - name: Build Release
      run: cargo build --release --verbose
"@.trim() | Set-Content "..\.github\workflows\$filename.yml"
  }
}
finally {
  Pop-Location
}