FILE=target/avr-none/release/attiny402.elf
objcopy -O ihex $FILE output.hex
"/home/tom/.arduino15/packages/megaTinyCore/tools/python3/3.7.2-post1/python3" -u "/home/tom/.arduino15/packages/megaTinyCore/hardware/megaavr/2.6.10/tools/prog.py"  -t uart -u /dev/ttyUSB0 -b 230400 -d attiny402 --fuses 0:0b00000000 2:0x01 6:0x04 7:0x00 8:0x00 "-foutput.hex" -a write -v
#arduino-cli upload --fqbn DxCore:megaavr:avrdb:chip=avr128db28,flmap=lockdefault,mvio=enabled,appspm=no,resetpin=reset,clock=8internal,millis=tcb2,startuptime=8,bodvoltage=1v9,bodmode=sampledslow,eesave=enable,printf=default,wiremode=mors,WDTtimeout=disabled,WDTwindow=disabled,attach=allenabled -p /dev/ttyUSB0 -P serialupdi
