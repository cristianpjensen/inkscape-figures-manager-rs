pub struct Style<'a> {
    pub fill_color: Option<&'a str>,
    pub fill_opacity: Option<f32>,
    pub stroke_color: Option<&'a str>,
    pub stroke_width: Option<StrokeThickness>,
    pub stroke_dash: Option<StrokeDash>,
    pub marker_start: Option<bool>,
    pub marker_end: Option<bool>,
}

impl Style<'_> {
    pub fn new() -> Self {
        Style {
            fill_color: None,
            fill_opacity: None,
            stroke_color: None,
            stroke_width: None,
            stroke_dash: None,
            marker_start: None,
            marker_end: None,
        }
    }
}

impl ToString for Style<'_> {
    fn to_string(&self) -> String {
        // Add styles
        let mut style_string = "stroke-linejoin:miter;strike-linecap:butt;".to_string();

        if let Some(color) = self.fill_color {
            style_string += &format!("fill:{};", color);
        }

        if let Some(opacity) = self.fill_opacity {
            style_string += &format!("fill-opacity:{};", opacity);
        }

        if let Some(color) = self.stroke_color {
            style_string += &format!("stroke:{};", color);
        }

        if let Some(width) = &self.stroke_width {
            style_string += &format!("stroke-width:{};", width.pts());
        }

        if let Some(dash) = &self.stroke_dash {
            style_string += &format!("stroke-dasharray:{};", dash.dasharray());
        }

        if let Some(marker_start) = self.marker_start {
            if marker_start {
                style_string += "marker-start:url(#ArrowWide);";
            } else {
                style_string += "marker-start:none;";
            }
        }

        if let Some(marker_end) = self.marker_end {
            if marker_end {
                style_string += "marker-end:url(#ArrowWide);";
            } else {
                style_string += "marker-end:none;";
            }
        }

        // Make SVG string so we can copy-paste the style onto the object
        let mut svg_string =
            "<?xml version='1.0' encoding='UTF-8' standalone='no'?><svg>".to_string();

        if self.marker_end.is_some_and(|x| x) || self.marker_start.is_some_and(|x| x) {
            svg_string += "
                <defs id='marker-defs'>
                    <marker
                    id='ArrowWide'
                    style='overflow:visible'
                    refX='0.46' refY='0'
                    orient='auto-start-reverse'
                    markerWidth='1'
                    markerHeight='1'
                    viewBox='0 0 1 1'
                    preserveAspectRatio='xMidYMid'
                    >
                        <path
                            style='fill:none;stroke:context-stroke;stroke-width:inherit;stroke-linecap:butt;'
                            d='M 3,-3 0,0 3,3'
                            transform='rotate(180,0.125,0)'
                        />
                    </marker>
                </defs>
            ";
        }

        svg_string + &format!("<inkscape:clipboard style='{style_string}' /></svg>")
    }
}

pub enum StrokeThickness {
    Normal,
    Thick,
    VeryThick,
}

impl StrokeThickness {
    fn pts(&self) -> f32 {
        match self {
            Self::Normal => 0.5308,
            Self::Thick => 1.0616,
            Self::VeryThick => 1.5924,
        }
    }
}

pub enum StrokeDash {
    Solid,
    Dashed,
    Dotted,
}

impl StrokeDash {
    fn dasharray(&self) -> &str {
        match self {
            Self::Solid => "none",
            Self::Dashed => "3.981,3.981",
            Self::Dotted => "0.5308,2.654",
        }
    }
}
