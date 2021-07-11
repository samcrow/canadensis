#!/usr/bin/env ruby

def round_to_primitive(bits)
    if bits <= 8
        'u8'
    elsif bits <= 16
        'u16'
    elsif bits <= 32
        'u32'
    elsif bits <= 64
        'u64'
    else
        raise 'Invalid number of bits'
    end
end

BIT_NUMBERS = 1..64

read_functions = BIT_NUMBERS.map do |i|
    primitive_type = round_to_primitive(i)
    "/// Reads a #{i}-bit unsigned integer
#[inline]
pub fn read_u#{i}(&mut self) -> #{primitive_type} { self.read_up_to_#{primitive_type}(#{i}) }"
end

skip_functions = BIT_NUMBERS.map do |i|
    if i == 1
        bit_or_bits = 'bit'
    else
        bit_or_bits = 'bits'
    end
    "/// Advances the cursor to skip #{i} #{bit_or_bits}
#[inline]
pub fn skip_#{i}(&mut self) { self.advance_bits(#{i}) }"
end

write_functions = BIT_NUMBERS.map do |i|
    primitive_type = round_to_primitive(i)
    "/// Writes a #{i}-bit unsigned integer
#[inline]
pub fn write_u#{i}(&mut self, value: #{primitive_type}) { self.write_up_to_#{primitive_type}(value, #{i}) }"
end

write_skip_functions = BIT_NUMBERS.map do |i|
    if i == 1
        bit_or_bits = 'bit'
    else
        bit_or_bits = 'bits'
    end
    "/// Advances the cursor to skip #{i} #{bit_or_bits}
#[inline]
pub fn skip_#{i}(&mut self) { self.skip_bits(#{i}); }"
end

# puts read_functions.join("\n")
# puts skip_functions.join("\n")
# puts write_functions.join("\n")
puts write_skip_functions.join("\n")
