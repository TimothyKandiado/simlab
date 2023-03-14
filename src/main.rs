use simlab::{flowsheet::Flowsheet, 
    stream::{material::Material, stream_builder::StreamBuilder, Stream}, 
    unit::{mixer::Mixer, Unit}};

fn main() {
    let mut flowsheet = Flowsheet::new();

    let material_1 = Material {mass: 12.0, name: "Water".to_string()};
    let material_2 = Material {mass: 8.0, name: "ethanol".to_string()};

    let stream_1 = StreamBuilder::new().add_material(material_1)
                                    .set_temperature(313.0)
                                    .set_pressure(3.0).build();

    let stream_2 = StreamBuilder::new().add_material(material_2)
                                    .set_pressure(4.5)
                                    .set_temperature(350.0)
                                    .build();

    let stream_3 = Stream::output_stream();

    let mixer = Mixer::new(vec!["stream 1".to_string(), "stream 2".to_string()], "stream 3".to_string());
    let mixer_unit: Box<dyn Unit> = Box::new(mixer);

    flowsheet = flowsheet.add_stream("stream 1".to_string(), stream_1)
                        .add_stream("stream 2".to_string(), stream_2)
                        .add_stream("stream 3".to_string(), stream_3)
                        .add_unit("mixer 01".to_string(), mixer_unit);

    flowsheet.run();
    let stream_results = flowsheet.get_all_streams();
    println!("Stream Results: \n{:?}", stream_results);
}
