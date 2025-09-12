include <../constants/motorMount.scad>
include <../libraries/sprockets/Sprockets.scad>
module fabric_feeder () {
    // Fabric feeder pole

    color("silver")
    translate([0,0,30])
    rotate([0,90,0])
	    sprocket (hub_diameter = 10, size = 1);
    
    color("silver")
    translate([0,0,fabric_arm_height-10])
    rotate([0,90,0])
	    sprocket (hub_diameter = 10, size = 1);
    
    arm_dimensions = [motor_mount_fabric_arm_width, motor_mount_fabric_arm_width, fabric_arm_height];
    arm_translation = 0+motor_mount_fabric_arm_width/2;

    
    color("white")
    translate([-arm_translation,0,fabric_arm_height/2])
        cube(arm_dimensions, center=true);

    color("white")
    translate([arm_translation+3,0,fabric_arm_height/2])
        cube(arm_dimensions, center=true);
}

module motor_mount() {
    color(motor_mount_colour)
    difference() {
        cube([motor_mount_width, motor_mount_height, motor_mount_thickness], center=true);

        // Axle hole
        translate([motor_mount_axel_offset_x, motor_mount_axel_offset_y, -motor_mount_thickness]) {
            cylinder(h = motor_mount_thickness*2, r = motor_mount_axel_radius, $fn=100);
        }

    }
    
    // Fabric pole
    color(motor_mount_colour)
    translate([motor_mount_fabric_pole_offset_x, motor_mount_fabric_pole_offset_y, motor_mount_thickness/2]) {
        cylinder(h = motor_mount_fabric_pole_height, r = motor_mount_fabric_pole_radius, $fn=100);
    }
}