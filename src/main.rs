use sdl2::EventPump;
use tokio::task;
use tokio::sync::mpsc;
use sdl2::event::Event;
use sdl2::controller::{Axis, GameController};
use sdl2::haptic::Haptic;
use tokio::net::UdpSocket;

mod payload;
use payload::payload::Payload;

fn sdl2_init() -> (GameController, Haptic, EventPump) {
    let sdl_context = sdl2::init().unwrap();

    let haptic_subsystem = sdl_context.haptic().unwrap();

    let controller_subsystem = sdl_context.game_controller().unwrap();
    
    controller_subsystem.set_event_state(true);

    let game_controller = match controller_subsystem.open(0) {
        Ok(controller) => controller,
        Err(_) => {
            println!("Failed to open game controller");
            panic!("no game controller")
        }
    };

    let haptic = haptic_subsystem.open_from_joystick_id(0).unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (game_controller, haptic, event_pump)
}






#[tokio::main]
async fn main() {
    
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



