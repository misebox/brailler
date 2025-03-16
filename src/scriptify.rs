use flate2::write::GzEncoder;
use flate2::Compression;
use base64::{engine::general_purpose, Engine as _};
use std::{io::{self, Write}, os::unix::fs::OpenOptionsExt};

fn gzip_and_base64_encode(input: &str) -> io::Result<String> {
    // Gzip 圧縮
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes())?;
    let compressed_data = encoder.finish()?; // Gzip 圧縮結果を取得

    // Base64 エンコード
    let encoded = general_purpose::STANDARD.encode(compressed_data);
    Ok(encoded)
}

pub static BASH_TEMPLATE_FOR_IMAGE: &str = r#"#!/bin/bash

# BRAILLE_TEXT START
BRAILLE_TEXT=$(cat <<'EOF' | base64 -d | gzip -d
{{OUTPUT}}
EOF
)
# BRAILLE_TEXT END

echo "$BRAILLE_TEXT"
"#;

pub static BASH_TEMPLATE_FOR_VIDEO: &str = r#"#!/bin/bash

# BRAILLE_TEXT START
BRAILLE_TEXT=$(cat <<'EOF' | base64 -d | gzip -d
{{OUTPUT}}
EOF
)
# BRAILLE_TEXT END

buffer=""
declare -a array

while IFS= read -r line; do
    if [[ "$line" == *,* ]]; then
        array+=("$buffer")
        buffer="${line#*,}"
    else
        if [[ -n "$buffer" ]]; then
            buffer+=$'\n'
        fi
        buffer+="$line"
    fi
done <<< "$BRAILLE_TEXT"

if [[ -n "$buffer" ]]; then
    array+=("$buffer")
fi


ESC=$(printf '\e')
for item in "${array[@]}"; do
    echo "${ESC}$item"
    sleep {{SLEEP}}
done
"#;

pub fn generate_bash_script_for_image(output: &str) -> io::Result<String> {
    let encoded = gzip_and_base64_encode(output)?;
    let script = BASH_TEMPLATE_FOR_IMAGE.replace("{{OUTPUT}}", &encoded);
    Ok(script)
}

pub fn generate_bash_script_for_video(output: &str, sleep: f32) -> io::Result<String> {
    let encoded = gzip_and_base64_encode(output)?;
    let script = BASH_TEMPLATE_FOR_VIDEO.replace("{{OUTPUT}}", &encoded).replace("{{SLEEP}}", &sleep.to_string());
    Ok(script)
}

pub fn save_script(script: &str, path: &str) -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o755)
        .open(path)?;
    file.write_all(script.as_bytes())?;
    file.sync_all()?;
    Ok(())
}
