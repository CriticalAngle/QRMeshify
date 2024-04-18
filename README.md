# QR Meshify
This program turns QR code image into an STL file, ready for print.

`NOTE: This software can only handle the basic QR codes that are simply comprised of squares. Any custom shapes, logos in the center, etc. will result in undesirable meshes. The squares can be different colors, but there must only be two (usually black and white).`

# Installation
Download the software [here](https://github.com/CriticalAngle/QRMeshify/releases/) in the releases tab

# Usage
1. Download the software
2. Find the image you would like to use as input
3. Open command prompt/terminal at the directory of the software
4. Run the software with the ABSOLUTE PATH of the image as an argument.

### Example
`./qr_meshify /Users/default/my_image.png`

5. The software will ask you for how many pixels an individual cell is (e.g., 18px).
6. It will then detect which two colors are present and prompt you to type in the same of the color that should be the mesh. The other color will simply be void.
7. Hit enter and you will have an image named qrcode.stl at the same path as the software.

Alternatively, you can input the pixel size and color as parameters in addition to the path of the image, like so: `./qr_meshify /Users/default/my_image.png 18 white`

## Issues:
If you need help, open an issue on GitHub or contact me at `support@criticalanglestudios.com`
