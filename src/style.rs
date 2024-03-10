pub struct Style<'a> {
    pub fill_color: &'a str,
    pub fill_opacity: f32,
    pub stroke_width: StrokeThickness,
    pub stroke_dash: StrokeDash,
    pub marker_start: bool,
    pub marker_end: bool,
}

impl Style<'_> {
    pub const fn new() -> Self {
        Style {
            fill_color: "none",
            fill_opacity: 1.0,
            stroke_width: StrokeThickness::None,
            stroke_dash: StrokeDash::None,
            marker_start: false,
            marker_end: false,
        }
    }
}

const POINTS: f32 = 1.327;

impl Style<'_> {
    pub fn to_string(&mut self) -> String {
        // Add styles
        let mut style_string =
            "stroke-linejoin:miter;strike-linecap:butt;stroke-opacity:1;".to_string();

        // Make sure there is a stroke if there is a dash or arrow
        if self.stroke_width == StrokeThickness::None
            && (self.stroke_dash != StrokeDash::None || self.marker_start || self.marker_end)
        {
            self.stroke_width = StrokeThickness::Normal;
        }

        // Set stroke color. If no stroke, set to none.
        if self.stroke_width == StrokeThickness::None {
            style_string += "stroke:none;";
        } else {
            style_string += "stroke:black;";
        }

        // Set attributes
        style_string += &format!("fill:{};", self.fill_color);
        style_string += &format!("stroke-width:{};", self.stroke_width.pts_string());
        style_string += &format!("fill-opacity:{};", self.fill_opacity);
        style_string += &format!(
            "stroke-dasharray:{};",
            self.stroke_dash.dasharray(self.stroke_width.pts())
        );

        // Set markers if needed
        if self.marker_start {
            style_string += &format!("marker-start:url(#tikz-arrow-{});", self.stroke_width.pts());
        } else {
            style_string += "marker-start:none;";
        }

        if self.marker_end {
            style_string += &format!("marker-end:url(#tikz-arrow-{});", self.stroke_width.pts());
        } else {
            style_string += "marker-end:none;";
        }

        // Make SVG string so we can copy-paste the style onto the object
        let mut svg_string =
            "<?xml version='1.0' encoding='UTF-8' standalone='no'?><svg>".to_string();

        if self.marker_end || self.marker_start {
            // Exact values of markers are taken from the default TikZ arrow
            svg_string += &format!("
                <defs id='marker-defs'>
                    <marker
                        id='tikz-arrow-{}'
                        orient='auto-start-reverse'
                        refX='{}' refY='0'
                        markerWidth='0.911' markerHeight='1.69'
                    >
                        <g transform='scale({})'>
                            <path
                                style='fill:none;stroke:#000000;stroke-width:0.6;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1'
                                d='M -1.55415,2.0722 C -1.42464,1.29512 0,0.1295 0.38852,0 0,-0.1295 -1.42464,-1.29512 -1.55415,-2.0722'
                                inkscape:connector-curvature='0'
                            />
                        </g>
                    </marker>
                </defs>
            ", self.stroke_width.pts(), match self.stroke_width { StrokeThickness::None | StrokeThickness::Normal => 0.45, StrokeThickness::Thick => 0.4, StrokeThickness::VeryThick => 0.2 }, (2.4 * self.stroke_width.pts() + 3.87) / (4.5 * self.stroke_width.pts()));
        }

        svg_string + &format!("<inkscape:clipboard style='{style_string}' /></svg>")
    }
}

#[derive(PartialEq)]
pub enum StrokeThickness {
    None,
    Normal,
    Thick,
    VeryThick,
}

impl StrokeThickness {
    fn pts(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Normal => POINTS * 0.4,
            Self::Thick => POINTS * 0.8,
            Self::VeryThick => POINTS * 1.2,
        }
    }

    fn pts_string(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Normal => self.pts().to_string(),
            Self::Thick => self.pts().to_string(),
            Self::VeryThick => self.pts().to_string(),
        }
    }
}

#[derive(PartialEq)]
pub enum StrokeDash {
    None,
    Solid,
    Dashed,
    Dotted,
}

impl StrokeDash {
    fn dasharray(&self, width: f32) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Solid => "none".to_string(),
            Self::Dashed => format!("{},{}", POINTS * 3.0, POINTS * 3.0),
            Self::Dotted => format!("{},{}", width, POINTS * 2.0),
        }
    }
}
