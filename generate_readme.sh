#!/bin/bash

IMG_01=samples/Vermeer_The_Girl_With_The_Pearl_Earring_B.jpg
IMG_02=samples/Munch_Scream.jpg

CODE='`'
CODE_BLOCK='```'

cd $(dirname $0)
cargo install --path .

cat >README.md <<EOD
# brailler

This tool converts images into Braille dot pattern text.

## Usage

${CODE_BLOCK}
\$ brailler --help
$( brailler --help )
${CODE_BLOCK}

Example:

${CODE_BLOCK}
\$ brailler $IMG_01 
$( brailler $IMG_01 )
${CODE_BLOCK}

## size

${CODE}ex: brailler $IMG_01 --size {cols or 0}x{rows or 0}${CODE}

This option allows you to specify the size of the output.
Sizes are specified in the form of "{cols}x{rows}".
Specify two numbers separated by "," or "x".
If you specify zero for one side, the size will be based on the image ratio.
If not specified or specified 0x0, it will behave the same as if 0x40 was specified.


${CODE_BLOCK}
\$ brailler $IMG_01 --size 0x20
$( brailler $IMG_01 --size 0x20)
${CODE_BLOCK}


## Preprocess

### contrast

- ${CODE}--contrast stretch${CODE} option allows you to preprocess the image with Contrast Stretch.
- ${CODE}--contrast equalize${CODE} option allows you to preprocess the image with Histogram Equalization.

### invert

- ${CODE}--invert${CODE} option allows you to invert the image.


${CODE_BLOCK}
paste -d' '  \\
  <( brailler $IMG_01 --size 50x0 ) \\
  <( brailler $IMG_01 --size 50x0 --contrast stretch ) \\
  <( brailler $IMG_01 --size 50x0 --contrast equalize ) \\
  <( brailler $IMG_01 --size 50x0 --invert )
$( paste -d' '  \
  <( brailler $IMG_01 --size 50x0 ) \
  <( brailler $IMG_01 --size 50x0 --contrast stretch ) \
  <( brailler $IMG_01 --size 50x0 --contrast equalize ) \
  <( brailler $IMG_01 --size 50x0 --invert ) \
)
${CODE_BLOCK}

### Binarize

- ${CODE}--binarize odith${CODE} option allows you to binarize the image with Ordered Dithering.
- ${CODE}--binarize fsdith${CODE} option allows you to binarize the image with Floyd-Steinberg Dithering.
- ${CODE}--binarize otsu${CODE} option allows you to binarize the image with Otsu's method.

${CODE_BLOCK}
paste -d' '  \\
  <( brailler $IMG_02 --size 50x0 ) \\
  <( brailler $IMG_02 --size 50x0 --binarize odith ) \\
  <( brailler $IMG_02 --size 50x0 --binarize fsdith ) \\
  <( brailler $IMG_02 --size 50x0 --binarize otsu )
$( paste -d' '  \
  <( brailler $IMG_02 --size 50x0 ) \
  <( brailler $IMG_02 --size 50x0 --binarize odith ) \
  <( brailler $IMG_02 --size 50x0 --binarize fsdith ) \
  <( brailler $IMG_02 --size 50x0 --binarize otsu ) \
)
${CODE_BLOCK}


EOD
