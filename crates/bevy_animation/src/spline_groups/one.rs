use crate::spline_group::{LoopStyle, SplineGroup};
use splines::Spline;

pub struct AnimationSplineOne {
    pub spline: Spline<f32, f32>,
    pub loop_style: LoopStyle,
    pub time: f32,
    pub speed: f32,
    pub paused: bool,
    pub pong: bool,
}

impl Default for AnimationSplineOne {
    fn default() -> Self {
        Self {
            spline: Spline::from_vec(vec![]),
            loop_style: LoopStyle::Once,
            time: 0.0,
            speed: 1.0,
            paused: false,
            pong: false,
        }
    }
}

impl SplineGroup for AnimationSplineOne {
    type Sample = Option<f32>;

    fn spline_key_times(&self) -> Vec<Box<dyn DoubleEndedIterator<Item = f32> + '_>> {
        vec![Box::new(self.spline.keys().iter().map(|key| key.t))]
    }

    fn loop_style(&self) -> LoopStyle {
        self.loop_style
    }

    fn loop_style_mut(&mut self) -> &mut LoopStyle {
        &mut self.loop_style
    }

    fn time(&self) -> f32 {
        self.time
    }

    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }

    fn paused(&self) -> bool {
        self.paused
    }

    fn paused_mut(&mut self) -> &mut bool {
        &mut self.paused
    }

    fn pong(&self) -> bool {
        self.pong
    }

    fn pong_mut(&mut self) -> &mut bool {
        &mut self.pong
    }

    fn sample(&self, time: f32) -> Self::Sample {
        self.spline.sample(time)
    }
}
