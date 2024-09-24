# raspberry_pi
Personnal projects around learning how to use Raspberry Pi and some lessons content

# Setup on Windows

1. Get the Prebuild Windows Toolchain for Raspberry Pi on this link : https://gnutoolchains.com/raspberry64/

2. Install the GNU toolchain for Raspberry Pi.

3. Run the command 'rustup target add aarch64-unknown-linux-gnu'

4. Create fil .cargo/config with the following content:

[build]
target = "aarch64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

5. Use WinScp to drag and drop the compiled file on Raspberry Pi.

