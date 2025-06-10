FILE=target/avr-none/release/avr_modern_demo.elf
objcopy -O ihex $FILE output.hex
"$HOME/.arduino15/packages/megaTinyCore/tools/python3/3.7.2-post1/python3" -u "$HOME/.arduino15/packages/DxCore/hardware/megaavr/1.5.11/tools/prog.py"  -t uart -u /dev/ttyUSB0 -b 230400 -d avr128db28 --fuses 6:0b00001100 7:0x00 8:0x00 "-foutput.hex" -a write -v

