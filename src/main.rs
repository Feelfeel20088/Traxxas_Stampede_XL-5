
use sdl2::EventPump;
use tokio::task;
use tokio::sync::mpsc;
use sdl2::event::Event;
use sdl2::controller::{Axis, GameController};
use sdl2::haptic::Haptic;
use tokio::net::UdpSocket;

mod payload;
use payload::payload::Payload;
use std::cell::BorrowError;
use std::process::Command;


fn sdl2_init() -> (GameController, Haptic, EventPump) {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");

    let haptic_subsystem = sdl_context.haptic().expect("Failed to get SDL2 haptic subsystem");

    let controller_subsystem = sdl_context.game_controller().expect("Failed to get SDL2 game controller subsystem");
    
    controller_subsystem.set_event_state(true);

    let game_controller = controller_subsystem
        .open(0)
        .expect("Failed to open game controller: no controller found or not supported");

    let haptic = haptic_subsystem
        .open_from_joystick_id(0)
        .expect("Failed to open haptic device from joystick");

    let event_pump = sdl_context.event_pump().expect("Failed to get SDL2 event pump");

    (game_controller, haptic, event_pump)
}


pub fn is_connected_to_rc() -> bool {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("nmcli")
            .args(&["-t", "-f", "active,ssid", "dev", "wifi"])
            .output()
            .expect("Failed to execute nmcli");

        let output = String::from_utf8_lossy(&output.stdout);
        // print!("{}", output);
        for line in output.lines() {
            if line.contains("yes:") {
                let ssid = &line[4..line.len()]; // Get SSID after "yes:"
                if ssid == "RC Car" {
                    return true; // Return SSID if it matches "RC Car"
                }
            }
        }
        false
    }

}





#[tokio::main]
async fn main() {


    if !is_connected_to_rc() {
        println!(
            "You're not connected to the 'RC_CAR' Wi-Fi network.\n\
            Please connect to the RC_CAR network and restart the program."
        );
        std::process::exit(1);
    }

    
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap(); 

    let (tx, mut rx) = mpsc::channel::<Payload>(32);

    let _ = task::spawn(async move {
        while let Some(object) = rx.recv().await {
            if let Err(e) = socket.send_to(&object.to_binary(), "192.168.1.1:1337").await {
                eprintln!("Failed to send UDP packet: {}", e);
            } else {
                println!("Sent: {:?}", object); // Requires Payload to derive Debug
            }
        }
    });


    let (game_controller, mut haptic, mut event_pump) = sdl2_init();

    haptic.rumble_play(1.0, 1000);
    
    
    
    

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // You can now check the controller's buttons and axes
        // if game_controller.button(sdl2::controller::Button::A) {
        //     println!("Button A is pressed");
        // }

        let left_x = game_controller.axis(Axis::LeftX);
        let trigger_right = game_controller.axis(Axis::TriggerRight);

        let pwm_joy_x: u16 = Payload::map_range(left_x, -32768,  32767, 180, 0);
        let pwm_trigger_right = Payload::map_range(trigger_right, 0,  32767,1500, 1000);

        let _ = tx.send(Payload::new(pwm_joy_x, pwm_trigger_right)).await;


        // println!("Left joystick: X = {}, Y = {} | Right joystick: X = {}, Y = {} | Triggers: Left = {}, Right = {} | joy_x = {}, trigger_right = {}", left_x, left_y, right_x, right_y, trigger_left, trigger_right, PWM_joy_x, PWM_trigger_right);

    }
        

        
        

    
}