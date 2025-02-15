
#[allow(dead_code)] // allows dead code, i.e. even if unused, will not give warning
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //allow debug, clone, copy, comparing 2 colors
#[repr(u8)] // representation of each value in u8(unsigned 8 bit-> ASCII) format
// create enum with public visibility
// enum with multiple variants
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/*
make this wrapper type disappear when it comes to memory layout
In this case, ColorCode will have exactly the same memory layout and ABI (Application Binary Interface) as a u8.
It's as if the struct wrapper doesn't exist from a memory perspective.
The rules are:

1. Can only have one field that takes up actual space in memory (non-zero-sized)
2. Can have additional fields that don't take up space (zero-sized)
3. Can only be used on structs or enums with a single variant

*/
//To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute
#[repr(transparent)]
// represent full color byte: foreground color + background color
struct ColorCode(u8); // deriving debug and copy trait and making it comparable

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // left shift by 4 bits 
    }
}


// Since the field ordering in default structs is undefined in Rust, we need the repr(C) attribute. 
// It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering. 
// For the Buffer struct, we use repr(transparent) again to ensure that it has the same memory layout as its single field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/*
The writer will always write to the last line and shift lines up when a line is full (or on \n).
The column_position field keeps track of the current position in the last row.
The current foreground and background colors are specified by color_code and a reference to the VGA buffer is stored in buffer. 
Note that we need an explicit lifetime here to tell the compiler how long the reference is valid.
The 'static lifetime specifies that the reference is valid for the whole program run time (which is true for the VGA text buffer).
*/
pub struct Writer {
    column_position: usize,// keep track of last position 
    color_code: ColorCode,// current foreground and background color as reference for VGA text buffer
    buffer: &'static mut Buffer, // lifetime to specify, for how much time, this reference is valid.
}

// implementing struct
impl Writer {
    // adding methods to structs?
    pub fn write_byte(&mut self, byte: u8) {
        // match block
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row =  BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                // add the character to the buffer
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.column_position+= 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        // s: &str(reference of str) => traverse each byte
        for byte in s.bytes() {
            match byte {
                // if ASCII or new line => write byte to VGA text buffer
                //The VGA text buffer only supports ASCII and the additional bytes of code page 437.
                // 0x20 => space character
                // 0x7e => ~ character
                // 0xfe => ■ character for unprintable bytes
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {/* TODO */}
    
}


pub fn print_something() {
    let mut writer = Writer {
        column_position : 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        //First, we cast the integer 0xb8000 as a mutable raw pointer. Then we convert it to a mutable reference by dereferencing it (through *) and immediately borrowing it again (through &mut). This conversion requires an unsafe block, since the compiler can’t guarantee that the raw pointer is valid.
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    }; // mutable buffer => buffer memory mapped to VGA hardware, that's why the repr(transparent)

    //The b prefix creates a byte literal, which represents an ASCII character. 
        writer.write_byte(b'H');
        writer.write_string("ello ");
        writer.write_string("Wörld!");
}