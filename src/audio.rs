use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::io::Cursor;

const AMBIENCE: [&[u8]; 4] = [
    include_bytes!("../assets/sounds/birds.ogg"),
    include_bytes!("../assets/sounds/forest.ogg"),
    include_bytes!("../assets/sounds/sea.ogg"),
    include_bytes!("../assets/sounds/storm.ogg"),
];
const END: &[u8] = include_bytes!("../assets/sounds/squashed_tomato.mp3");

#[derive(Default)]
pub struct Audio {
    stream: Option<OutputStream>,
    ambient: Option<Sink>,
    end: Option<Sink>,
}

impl Audio {
    fn sink(&mut self) -> Result<Sink, String> {
        if self.stream.is_none() {
            self.stream =
                Some(OutputStreamBuilder::open_default_stream().map_err(|e| e.to_string())?);
        }
        Ok(Sink::connect_new(self.stream.as_ref().unwrap().mixer()))
    }

    pub fn ambient(&mut self, selection: i32) -> Result<(), String> {
        self.ambient = None;
        if let Some(sound) = selection
            .checked_sub(1)
            .and_then(|i| AMBIENCE.get(i as usize))
        {
            let source = Decoder::new(Cursor::new(*sound)).map_err(|e| e.to_string())?;
            let sink = self.sink()?;
            sink.set_volume(0.35);
            sink.append(source.repeat_infinite());
            self.ambient = Some(sink);
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), String> {
        let source = Decoder::new(Cursor::new(END)).map_err(|e| e.to_string())?;
        let sink = self.sink()?;
        sink.append(source);
        self.end = Some(sink);
        Ok(())
    }
}
