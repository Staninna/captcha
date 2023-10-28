use captcha::filters::{Dots as Dot, Grid, Noise, Wave};

#[derive(Debug)]
enum FilterType {
    Dot,
    Grid,
    Wave,
    Noise,
}

pub struct Filters {
    dots: Vec<Dot>,
    grids: Vec<Grid>,
    waves: Vec<Wave>,
    noises: Vec<Noise>,
    order: Vec<FilterType>,
}

impl Filters {
    pub fn new(filter_str: &str) -> Result<Self, String> {
        let mut filters = Self {
            dots: Vec::new(),
            grids: Vec::new(),
            waves: Vec::new(),
            noises: Vec::new(),
            order: Vec::new(),
        };
        filters.parse(filter_str)?;
        Ok(filters)
    }

    pub fn next_filter(&mut self, captcha: &mut captcha::Captcha) -> Result<(), String> {
        if self.order.is_empty() {
            return Err("No more filters to apply".to_string());
        }

        let filter_type = self.order.remove(0);
        dbg!(&filter_type);
        match filter_type {
            FilterType::Dot => {
                let dot = self.dots.remove(0);
                captcha.apply_filter(dot);
            }
            FilterType::Grid => {
                let grid = self.grids.remove(0);
                captcha.apply_filter(grid);
            }
            FilterType::Wave => {
                let wave = self.waves.remove(0);
                captcha.apply_filter(wave);
            }
            FilterType::Noise => {
                let noise = self.noises.remove(0);
                captcha.apply_filter(noise);
            }
        }

        Ok(())
    }

    fn parse(&mut self, filter_str: &str) -> Result<(), String> {
        let mut dots = Vec::new();
        let mut grids = Vec::new();
        let mut waves = Vec::new();
        let mut noises = Vec::new();
        let mut order = Vec::new();

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
                    order.push(FilterType::Dot);
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
                    order.push(FilterType::Grid);
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
                    order.push(FilterType::Wave);
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
                    order.push(FilterType::Noise);
                }
                _ => return Err(format!("Unknown filter type: {}", filter_type)),
            }
        }

        self.dots = dots;
        self.grids = grids;
        self.waves = waves;
        self.noises = noises;
        self.order = order;

        Ok(())
    }
}
