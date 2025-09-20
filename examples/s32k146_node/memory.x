MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  FLASH_CONFIGURATION_FIELD : ORIGIN = 0x0400, LENGTH = 16
  FLEXRAM : ORIGIN = 0x14000000, LENGTH = 4K
  SRAM_L : ORIGIN = 0x1fff0000, LENGTH = 64K
  /* SRAM_U */
  RAM : ORIGIN = 0x20000000, LENGTH = 60K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* Leave space for the flash configuration field after the vector table */
_stext = ORIGIN(FLASH) + 0x410;

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/

/* Warning: Changing this may permanently disable debugger access. Read the reference manual carefully. */
SECTIONS {
    .flash_configuration_field : {
        . = 0x400;
        /* Backdoor comparison key */
        LONG(0xffffffff)
        LONG(0xffffffff)
        /* Program flash protection bytes */
        LONG(0xffffffff)
        /* Flash security byte */
        BYTE(0xfe)
        /* Flash nonvolatile option byte */
        BYTE(0xff)
        /* EEPROM protection byte */
        BYTE(0xff)
        /* Data flash protection byte */
        BYTE(0xff)
    } > FLASH_CONFIGURATION_FIELD
}
