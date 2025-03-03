# Create project directory
mkdir gehri-os && cd gehri-os

# Create source directories 
mkdir -p src/arch/x86_64 src/kernel src/drivers

# Create initial files
touch src/main.rs
touch src/arch/x86_64/boot.asm
touch README.md 