#!/usr/bin/env ruby

MAX_REGISTERS = 32

(2..MAX_REGISTERS).each do |num_registers|
    register_range = 0...num_registers
    type_args = register_range.map{|n| "R#{n}"}.join(', ')
    match_arms = register_range.map{|n| "#{n} => Some(&self.#{n}),"}.join("\n    ")
    match_arms_mut = register_range.map{|n| "#{n} => Some(&mut self.#{n}),"}.join("\n    ")
    if_blocks = register_range.map{|n| "if name == self.#{n}.name() { Some(&mut self.#{n}) }"}.join("\n    else ")
    where_clauses = register_range.map{|n| "R#{n}: Register,"}.join("\n    ")

    puts "impl<#{type_args}> RegisterBlock for (#{type_args})
        where #{where_clauses} {
         fn register_by_index(&self, index: usize) -> Option<&dyn Register> {
             match index {
                 #{match_arms}
                 _ => None,
             }
         }
         fn register_by_index_mut(&mut self, index: usize) -> Option<&mut dyn Register> {
             match index {
                 #{match_arms_mut}
                 _ => None,
             }
         }
         fn register_by_name_mut(&mut self, name: &str) -> Option<&mut dyn Register> {
             #{if_blocks}
            else { None }
         }
     }"

end
