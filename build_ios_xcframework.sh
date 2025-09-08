#!/bin/bash
set -e

# Nom du projet / crate
CRATE_NAME="theme_numerologie_docx"

# Dossier de sortie
OUTPUT_DIR="ios_build"
XCFRAMEWORK="$OUTPUT_DIR/$CRATE_NAME.xcframework"

# Créer le dossier de sortie
rm -rf $OUTPUT_DIR
mkdir -p $OUTPUT_DIR

# Vérifier et créer le dossier include/ avec un header minimal si nécessaire
INCLUDE_DIR="include"
HEADER_FILE="$INCLUDE_DIR/$CRATE_NAME.h"
if [ ! -d "$INCLUDE_DIR" ]; then
  echo "Le dossier include/ est manquant. Création de $INCLUDE_DIR et d'un header minimal."
  mkdir -p "$INCLUDE_DIR"
fi

if [ ! -f "$HEADER_FILE" ]; then
  echo "Le fichier header $HEADER_FILE est manquant. Création d'un header minimal."
  cat > "$HEADER_FILE" <<EOF
#ifndef NUMEROLOGIE_EXCEL_H
#define NUMEROLOGIE_EXCEL_H

// Header minimal pour $CRATE_NAME

#endif // NUMEROLOGIE_EXCEL_H
EOF
fi

# Ajouter les targets iOS
rustup target add aarch64-apple-ios      # iOS device
rustup target add aarch64-apple-ios-sim  # Simulator Apple Silicon

# Compiler pour device arm64
cargo build --release --target aarch64-apple-ios
DEVICE_LIB="target/aarch64-apple-ios/release/lib${CRATE_NAME}.a"

# Compiler pour simulator arm64
cargo build --release --target aarch64-apple-ios-sim
SIM_ARM_LIB="target/aarch64-apple-ios-sim/release/lib${CRATE_NAME}.a"

# Vérifier que tous les fichiers .a existent
missing_libs=0
for lib in "$DEVICE_LIB" "$SIM_ARM_LIB"; do
  if [ ! -f "$lib" ]; then
    echo "Erreur : la bibliothèque $lib est manquante. Veuillez vérifier la compilation."
    missing_libs=1
  fi
done

if [ "$missing_libs" -eq 1 ]; then
  echo "Abandon de la création de l'XCFramework en raison de fichiers manquants."
  exit 1
fi

# Créer l'XCFramework
xcodebuild -create-xcframework \
  -library "$DEVICE_LIB" -headers "$INCLUDE_DIR" \
  -library "$SIM_ARM_LIB" -headers "$INCLUDE_DIR" \
  -output "$XCFRAMEWORK"

echo "XCFramework généré : $XCFRAMEWORK"