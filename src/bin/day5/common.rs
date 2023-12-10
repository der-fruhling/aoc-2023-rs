use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct ConversionMapEntry {
    pub source: Range<i64>,
    pub target: Range<i64>
}

impl ConversionMapEntry {
    pub fn offset(&self) -> i64 {
        self.target.start - self.source.start
    }
}

impl From<&str> for ConversionMapEntry {
    fn from(value: &str) -> Self {
        todo!()
    }
}

pub trait ConversionStep: Debug {
    fn lookup(&self, index: i64) -> i64;
}

#[derive(Debug)]
pub struct ConversionMapByRange {
    pub next_step: Arc<dyn ConversionStep + Sync + Send>,
    pub entries: Vec<ConversionMapEntry>
}

impl ConversionMapByRange {
    pub fn new() -> Self {
        Self {
            next_step: Arc::new(ConversionMapDummy),
            entries: Vec::new()
        }
    }

    pub fn with_next_step(self, next_step: Arc<dyn ConversionStep + Sync + Send>) -> Self {
        Self { next_step, ..self }
    }
}

impl ConversionStep for ConversionMapByRange {
    fn lookup(&self, index: i64) -> i64 {
        for entry in &self.entries {
            if entry.source.contains(&index) {
                return self.next_step.lookup(index + entry.offset());
            }
        }

        return self.next_step.lookup(index);
    }
}

#[derive(Debug)]
pub struct ConversionMapDummy;

impl ConversionStep for ConversionMapDummy {
    fn lookup(&self, index: i64) -> i64 {
        index
    }
}

#[derive(Debug)]
pub struct ConversionMapper {
    pub seeds: Vec<i64>,
    pub top: Arc<ConversionMapByRange>
}

impl ConversionMapper {
    pub fn lookup(&self, index: i64) -> i64 {
        self.top.lookup(index)
    }
}

impl From<&str> for ConversionMapper {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().filter(|v| !v.is_empty());
        let seed_list = &lines.next().unwrap()[7..];
        let seeds: Vec<_> = seed_list.split(' ').map(|v| i64::from_str(v).unwrap()).collect();

        let mut humidity_to_location = ConversionMapByRange::new();
        let mut temperature_to_humidity = ConversionMapByRange::new();
        let mut light_to_temperature = ConversionMapByRange::new();
        let mut water_to_light = ConversionMapByRange::new();
        let mut fertilizer_to_water = ConversionMapByRange::new();
        let mut soil_to_fertilizer = ConversionMapByRange::new();
        let mut seed_to_soil = ConversionMapByRange::new();

        // filler default value
        let mut conversion_map = &mut humidity_to_location;

        for line in lines {
            if line.ends_with("map:") {
                conversion_map = match line {
                    "humidity-to-location map:" => &mut humidity_to_location,
                    "temperature-to-humidity map:" => &mut temperature_to_humidity,
                    "light-to-temperature map:" => &mut light_to_temperature,
                    "water-to-light map:" => &mut water_to_light,
                    "fertilizer-to-water map:" => &mut fertilizer_to_water,
                    "soil-to-fertilizer map:" => &mut soil_to_fertilizer,
                    "seed-to-soil map:" => &mut seed_to_soil,
                    _ => unimplemented!()
                }
            } else {
                let &[data1, data2, data3] = line.split(' ').collect::<Vec<_>>().as_slice() else { unimplemented!("{}", line) };

                let dst = i64::from_str(data1).unwrap();
                let src = i64::from_str(data2).unwrap();
                let len = i64::from_str(data3).unwrap();

                conversion_map.entries.push(ConversionMapEntry {
                    source: src..(src + len),
                    target: dst..(dst + len)
                });
            }
        }

        let humidity_to_location = Arc::new(humidity_to_location);
        let temperature_to_location = Arc::new(temperature_to_humidity.with_next_step(humidity_to_location));
        let light_to_location = Arc::new(light_to_temperature.with_next_step(temperature_to_location));
        let water_to_location = Arc::new(water_to_light.with_next_step(light_to_location));
        let fertilizer_to_location = Arc::new(fertilizer_to_water.with_next_step(water_to_location));
        let soil_to_fertilizer = Arc::new(soil_to_fertilizer.with_next_step(fertilizer_to_location));
        let seed_to_location = Arc::new(seed_to_soil.with_next_step(soil_to_fertilizer));

        Self {
            seeds,
            top: seed_to_location
        }
    }
}
