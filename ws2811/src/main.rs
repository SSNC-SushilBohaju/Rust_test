use rs_ws281x::ControllerBuilder;
use rs_ws281x::controller::builder;
use rs_ws281x::controller::builder::ControllerBuilder;

fn main() {
    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(12)
                .count(1) // Number of LEDs
                .strip_type(StripType::Ws2811Rgb)
                .brightness(50) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = [0, 0, 255, 0];
    }

    controller.render().unwrap();
}