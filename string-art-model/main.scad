
include <models/nailBoard.scad>
include <models/motorMount.scad>
include <models/fabricFeeder.scad>

include <constants/nailBoard.scad>
include <constants/units.scad>

translate([motor_mount_axel_offset_x, motor_mount_axel_offset_y, nail_board_thickness/2+2*mm])
    nail_board_with_nails();
motor_mount();  

translate([motor_mount_axel_offset_x, -motor_mount_axel_offset_y, nail_board_thickness/2+2*mm])
    fabric_feeder();