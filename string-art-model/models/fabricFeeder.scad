include <../constants/fabricFeeder.scad>
include <../libraries/sprockets/Sprockets.scad>
include <../libraries/MCAD/motors.scad>

module _fabric_feeder_arm(arm_translation_x = 0) {
    //structural arm on the sides
    color("white")
    difference() {
        translate([arm_translation_x+5,0,fabric_arm_height/2])
            cube(arm_dimensions, center=true);

        translate([arm_translation_x-10,0,fabric_arm_top_height])
            rotate([0,90,0])
                cylinder(h = fabric_arm_width+10, r = fabric_arm_axel_radius+1, $fn=100);
        
        
        translate([arm_translation_x-10,0,fabric_arm_height-10])
            rotate([0,90,0])
                cylinder(h = fabric_arm_width+10, r = fabric_arm_axel_radius+1, $fn=100);
    }
}

module _sprocket_gear(translation_z = 0) {
    //sprocket gear and axel
    color("silver")
    translate([0,0,translation_z])
    rotate([0,90,0])
        sprocket (hub_diameter = fabric_arm_axel_radius*2, size = 1);
    
    axel_translation_x = (fabric_arm_width+10);
    color("silver")
    translate([-axel_translation_x,0,translation_z])
        rotate([0,90,0])
            cylinder(h = axel_translation_x*2, r = fabric_arm_axel_radius, $fn=100);

}

module fabric_feeder () {
    _sprocket_gear(translation_z = fabric_arm_top_height);
    _sprocket_gear(translation_z = fabric_arm_height-10);
    
    for (i = [1 : 2]) {
        _fabric_feeder_arm(arm_translation_x =  i^i*10 - 28);
    }

    //stepper motor mount
    rotate([0,90,0])
    translate([-fabric_arm_width,0,-fabric_arm_width*2-10])
    stepper_motor_mount(nema_standard = 17);
    
}