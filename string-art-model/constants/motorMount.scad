include <units.scad>
include <nailBoard.scad>

motor_mount_colour = "grey";
motor_mount_width = 800*mm;
motor_mount_height = 600*mm;
motor_mount_thickness = 20*mm;


motor_mount_axel_radius = nail_board_axel_radius + 1*mm;
motor_mount_axel_offset_x = motor_mount_width*0.3;
motor_mount_axel_offset_y = motor_mount_height*0.3;

motor_mount_fabric_pole_radius = 4*mm;
motor_mount_fabric_pole_height = 30*mm;
motor_mount_fabric_pole_offset_x = -motor_mount_width*0.3;
motor_mount_fabric_pole_offset_y = -motor_mount_height*0.3;
