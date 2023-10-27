use captcha::filters::{Dots as Dot, Grid, Noise, Wave};

pub struct Filters {
    pub dots: Vec<Dot>,
    pub grids: Vec<Grid>,
    pub waves: Vec<Wave>,
    pub noises: Vec<Noise>,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            dots: Vec::new(),
            grids: Vec::new(),
            waves: Vec::new(),
            noises: Vec::new(),
        }
    }

    pub fn parse(&mut self, filter_str: &str) -> Result<(), String> {
        let mut dots = Vec::new();
        let mut grids = Vec::new();
        let mut waves = Vec::new();
        let mut noises = Vec::new();

        let filters = filter_str.split(';').collect::<Vec<&str>>();
        for filter in filters {
            if filter.is_empty() {
                continue;
            }

            let splitted = filter.split(':').collect::<Vec<&str>>();
            if splitted.len() != 2 {
                return Err(format!("Invalid filter format: {}", filter));
            }

            let filter_type = splitted[0];
            let filter_args = splitted[1].split(',').collect::<Vec<&str>>();

            match filter_type {
                "dot" => {
                    if filter_args.len() != 1 {
                        return Err(format!(
                            "Invalid number of arguments for 'dot' filter: {}",
                            filter
                        ));
                    }
                    let n = filter_args[0]
                        .parse::<u32>()
                        .map_err(|e| format!("Failed to parse 'dot' argument: {}", e))?;
                    let dot = Dot::new(n);
                    dots.push(dot);
                }
                "grid" => {
                    if filter_args.len() != 2 {
                        return Err(format!(
                            "Invalid number of arguments for 'grid' filter: {}",
                            filter
                        ));
                    }
                    let x_gap = filter_args[0]
                        .parse::<u32>()
                        .map_err(|e| format!("Failed to parse 'grid' X argument: {}", e))?;
                    let y_gap = filter_args[1]
                        .parse::<u32>()
                        .map_err(|e| format!("Failed to parse 'grid' Y argument: {}", e))?;
                    let grid = Grid::new(x_gap, y_gap);
                    grids.push(grid);
                }
                "wave" => {
                    if filter_args.len() != 3 {
                        return Err(format!(
                            "Invalid number of arguments for 'wave' filter: {}",
                            filter
                        ));
                    }
                    let f = filter_args[0]
                        .parse::<f64>()
                        .map_err(|e| format!("Failed to parse 'wave' frequency argument: {}", e))?;
                    let amp = filter_args[1]
                        .parse::<f64>()
                        .map_err(|e| format!("Failed to parse 'wave' amplitude argument: {}", e))?;
                    let direction = filter_args[2];
                    let wave = Wave::new(f, amp);
                    let wave = match direction {
                        "v" => wave.vertical(),
                        "h" => wave.horizontal(),
                        _ => return Err(format!("Unknown wave direction: {}", direction)),
                    };
                    waves.push(wave);
                }
                "noise" => {
                    if filter_args.len() != 1 {
                        return Err(format!(
                            "Invalid number of arguments for 'noise' filter: {}",
                            filter
                        ));
                    }
                    let prob = filter_args[0]
                        .parse::<f32>()
                        .map_err(|e| format!("Failed to parse 'noise' argument: {}", e))?;
                    let noise = Noise::new(prob);
                    noises.push(noise);
                }
                _ => return Err(format!("Unknown filter type: {}", filter_type)),
            }
        }

        self.dots = dots;
        self.grids = grids;
        self.waves = waves;
        self.noises = noises;

        Ok(())
    }
}
