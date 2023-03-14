use simlab::{flowsheet::Flowsheet, 
    stream::{material::Material, stream_builder::StreamBuilder, Stream}, 
    unit::{mixer::Mixer, Unit}};

fn main() {
    let mut flowsheet = Flowsheet::new();

    let material_1 = Material {mass: 12.0, name: "Water".to_string()};
    let material_2 = Material {mass: 8.0, name: "ethanol".to_string()};
    let material_3 = Material {mass: 5.0, name: "Benzeze".to_string()};
    let material_4 = Material {mass: 4.0, name: "Water".to_string()};

    let stream_1 = StreamBuilder::new().add_material(material_1)
                                    .set_temperature(313.0)
                                    .set_pressure(3.0).build();

    let stream_2 = StreamBuilder::new().add_material(material_2)
                                    .set_pressure(4.5)
                                    .set_temperature(350.0)
                                    .build();

    let stream_3 = Stream::output_stream();

    let stream_4 = StreamBuilder::new().add_material(material_3)
                                        .add_material(material_4)
                                        .set_pressure(6.0)
                                        .set_temperature(295.0)
                                        .build();

    let stream_5 = Stream::output_stream();

    let mixer = Mixer::new(vec!["stream 1".to_string(), "stream 2".to_string()], "stream 3".to_string());
    let mixer_unit: Box<dyn Unit> = Box::new(mixer);

    let mixer_2 = Mixer::new(vec!["stream 4".to_string(), "stream 3".to_string()], "stream 5".to_string());
    let mixer_unit_2: Box<dyn Unit> = Box::new(mixer_2);

    flowsheet = flowsheet.add_stream("stream 1".to_string(), stream_1)
                        .add_stream("stream 2".to_string(), stream_2)
                        .add_stream("stream 3".to_string(), stream_3)
                        .add_unit("mixer 01".to_string(), mixer_unit)
                        .add_stream("stream 4".to_string(), stream_4)
                        .add_stream("stream 5".to_string(), stream_5)
                        .add_unit("mixer 02".to_string(), mixer_unit_2);

    flowsheet.run();
    let stream_results = flowsheet.get_all_streams();
    for (name,stream) in stream_results.into_iter() {
        println!("{} \n{:?}", name, stream);
        println!();
    }
}
