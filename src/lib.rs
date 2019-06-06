#![no_std]

extern crate ev3rt;
use ev3rt::*;
extern crate ev3;
use ev3::*;

fn init(ev3: &mut Ev3) {
    ev3.screen.setup(ScreenOrientation::Up);

    //ev3.s1().configure(SensorConfiguration::Color(ColorSensorMode::REFLECT));
    ev3.s1().configure(SensorConfiguration::None);
    ev3.s2().configure(SensorConfiguration::None);
    ev3.s3().configure(SensorConfiguration::None);
    ev3.s4().configure(SensorConfiguration::None);

    // ev3.ma().configure(MotorType::UNDEGULATED);
    ev3.ma().configure(MotorType::NONE);
    ev3.mb().configure(MotorType::NONE);
    ev3.mc().configure(MotorType::NONE);
    ev3.md().configure(MotorType::NONE);

    //ev3.apply_configuration();
}

#[no_mangle]
pub extern "C" fn main_task() -> () {
    let mut ev3 = Ev3::new();
    init(&mut ev3);

    /*
    ev3.screen.set_info_count(1);

    ev3.screen.setup_info(0, 4, 1, 1, 1, 1);
    ev3.screen
        .setup_info_glyphs(0, &[Gph::Minus, Gph::Minus, Gph::Minus, Gph::Minus]);
    //ev3.screen
    //    .setup_info_glyphs(0, &[Gph::T, Gph::E, Gph::S, Gph::T]);

    ev3.screen.setup_info(1, 8, 2, 3, 1, 1);
    ev3.screen
        .setup_info_glyphs(1, &[Gph::C, Gph::O, Gph::L, Gph::Space]);
    ev3.screen.setup_info_value(1, 4, 4);

    ev3.screen.setup_info(2, 8, 3, 3, 1, 1);
    ev3.screen
        .setup_info_glyphs(2, &[Gph::P, Gph::W, Gph::R, Gph::Space]);
    ev3.screen.setup_info_value(2, 4, 4);
    */

    lcd_draw_string("LOOP", 2, 2);
    lcd_draw_line(2, 0, 50, 0);

    //let mut pwr = 0;
    let mut x = 0;
    loop {
        lcd_draw_string("LEAP", x + 2, x + 32);
        lcd_draw_line(x + 2, x + 30, x + 50, x + 30);

        x += 1;
        if x > 100 {
            x = 0;
        }
        usleep(100);

        ev3.read();

        /*
        if ev3.keys.back.release_event() {
            abort();
        }
        if ev3.keys.up.release_event() {
            pwr += 10;
            if pwr > 100 {
                pwr = 100;
            }
        }
        if ev3.keys.down.release_event() {
            pwr -= 10;
            if pwr < -100 {
                pwr = -100;
            }
        }

        if ev3.keys.right.is_pressed() {
            ev3.leds.set_red(true);
        } else if ev3.keys.left.is_pressed() {
            ev3.leds.set_green(true);
        } else {
            ev3.leds.reset();
        }
        */

        if button_is_pressed(Button::RIGHT) {
            led_set_color(LedColor::GREEN);
        } else if button_is_pressed(Button::LEFT) {
            led_set_color(LedColor::RED);
        } else {
            led_set_color(LedColor::OFF);
        }

        //let color = ev3.s1().val();
        //let color = 0;
        //ev3.screen.set_info_value(1, color);
        //ev3.screen.set_info_value(2, pwr);

        ev3.apply();
    }
}
