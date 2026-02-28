use crate::ui;
use plotters::coord::Shift;
use plotters::prelude::*;
// use plotters::style::Color as _;
use plotters_iced2::Chart;

pub struct ThumbChart<'a> {
    pub data: &'a [f64],
    pub color: RGBColor,
}

impl<'a> Chart<ui::Message> for ThumbChart<'a> {
    type State = ();

    fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>) {
        root.fill(&RGBColor(25, 25, 38)).unwrap();
        self.build_chart(state, ChartBuilder::on(&root));
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let x_max = if self.data.len() > 1 {
            (self.data.len() - 1) as f64
        } else {
            1.0
        };

        let mut chart = builder
            .margin(2)
            .build_cartesian_2d(0f64..x_max, 0f64..100f64)
            .expect("failed to build thumb chart");

        chart
            .configure_mesh()
            .disable_mesh()
            .disable_axes()
            .draw()
            .expect("failed to draw thumb mesh");

        if !self.data.is_empty() {
            chart
                .draw_series(LineSeries::new(
                    self.data.iter().enumerate().map(|(i, &v)| (i as f64, v)),
                    ShapeStyle::from(self.color).stroke_width(2),
                ))
                .expect("failed to draw thumb series");
        }
    }
}

pub struct DetailChart<'a> {
    pub data: &'a [f64],
    pub color: RGBColor,
    pub y_label: &'a str,
    pub max_size: f64,
}

impl<'a> Chart<ui::Message> for DetailChart<'a> {
    type State = ();

    fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>) {
        root.fill(&RGBColor(25, 25, 38)).unwrap();
        self.build_chart(state, ChartBuilder::on(&root));
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let x_max = if self.data.len() > 1 {
            (self.data.len() - 1) as f64
        } else {
            1.0
        };

        let mut chart = builder
            .x_label_area_size(30)
            .y_label_area_size(40)
            .margin(10)
            .build_cartesian_2d(0f64..x_max, 0f64..self.max_size)
            .expect("failed to build detail chart");

        chart
            .configure_mesh()
            .label_style(("sans-serif", 12, &WHITE))
            .bold_line_style(WHITE.mix(0.1))
            .light_line_style(WHITE.mix(0.05))
            .axis_style(WHITE.mix(0.3))
            .x_desc("Time (s)")
            .y_desc(self.y_label)
            .draw()
            .expect("failed to draw detail mesh");

        if !self.data.is_empty() {
            chart
                .draw_series(
                    AreaSeries::new(
                        self.data.iter().enumerate().map(|(i, &v)| (i as f64, v)),
                        0.0,
                        self.color.mix(0.2),
                    )
                    .border_style(ShapeStyle::from(self.color).stroke_width(2)),
                )
                .expect("failed to draw detail series");
        }
    }
}
