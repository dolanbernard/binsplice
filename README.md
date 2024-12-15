# bdbin

Binutil for dumping and patching binary files

## Usage

### Dumping Binaries

To dump a binary file to stdout, use the following command:

`bdbin dump -i <binary file>`

The output format can be configured using flags. See

`bdbin dump -h`

for a list.

### Patching Binaries

To patch a binary file, use the following command:

`bdbin patch -i <binary file> -p <patch file> -o <patched binary output file>`

The patch file must be a valid IPS patch file.
