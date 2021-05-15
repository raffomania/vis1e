use std::{collections::VecDeque, time::Duration};

use rayon::prelude::*;

use nannou::{prelude::*, rand::random_f32, ui::prelude::*};

fn main() {
    better_panic::install();
    nannou::app(model).update(update).run();
}

struct Agent {
    pos: Point2,
    dir: Vector2,
    history: VecDeque<Vector2>,
}

struct Config {
    trail_length: u8,
}

struct Model {
    agents: VecDeque<Agent>,
    window: window::Id,
    ui: Ui,
    ids: Ids,
    config: Config,
}

widget_ids! {
    struct Ids {
        fps,
        agent_count
    }
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    let agents = VecDeque::with_capacity(100000);

    Model {
        agents,
        window,
        ui,
        ids,
        config: Config { trail_length: 10 },
    }
}

fn random_agent(config: &Config) -> Agent {
    let r1 = random_f32().max(0.2);
    let r2 = random_f32().max(0.2);
    let dir = (vec2(r1, r2) - vec2(0.5, 0.5)) * 500.0;
    Agent {
        pos: pt2(0.0, 0.0),
        dir,
        history: VecDeque::with_capacity(config.trail_length.to_usize().unwrap()),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let bounds = app.window(model.window).unwrap().rect();

    model.agents.push_back(random_agent(&model.config));

    if model.agents.len() > 1000 {
        model.agents.pop_front();
    }

    let dt = update.since_last.clone();
    let config = &model.config;
    let frame = app.elapsed_frames();
    model
        .agents
        .par_iter_mut()
        .for_each(|agent| update_agent(agent, config, &dt, &bounds, frame));

    let ui = &mut model.ui.set_widgets();

    let fps = format!("{} fps", app.fps().floor());
    widget::TextBox::new(&fps)
        .align_top()
        .w_h(200.0, 25.0)
        .text_color(nannou::ui::Color::Rgba(1.0, 1.0, 1.0, 1.0))
        .rgb(0.0, 0.0, 0.0)
        .set(model.ids.fps, ui);

    let agent_count = format!("{} agents", model.agents.len());
    widget::TextBox::new(&agent_count)
        .align_top()
        .w_h(200.0, 25.0)
        .text_color(nannou::ui::Color::Rgba(1.0, 1.0, 1.0, 1.0))
        .rgb(0.0, 0.0, 0.0)
        .down(10.0)
        .set(model.ids.agent_count, ui);
}

fn update_agent(
    agent: &mut Agent,
    config: &Config,
    dt: &Duration,
    bounds: &nannou::prelude::Rect,
    frame_number: u64,
) {
    // if (frame_number % 10) == 0 {
    agent.history.push_front(agent.pos);
    agent.history.truncate(config.trail_length.into());
    // }

    agent.pos += agent.dir * dt.as_secs_f32();

    if (agent.pos.x < bounds.left() && agent.dir.x < 0.0)
        || (agent.pos.x > bounds.right() && agent.dir.x > 0.0)
    {
        agent.dir.x *= -1.0;
    }
    if (agent.pos.y < bounds.bottom() && agent.dir.y < 0.0)
        || (agent.pos.y > bounds.top() && agent.dir.y > 0.0)
    {
        agent.dir.y *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    for agent in &model.agents {
        draw_agent(&draw, &agent)
    }

    draw.to_frame(app, &frame).unwrap();

    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn draw_agent(draw: &Draw, agent: &Agent) {
    let trail_length = agent.history.len();
    let points = agent.history.iter().enumerate().map(move |(i, pt)| {
        let intensity = 1.1 - (i.to_f32().unwrap() / trail_length.to_f32().unwrap());
        let color = rgba(1.0, 1.0, 1.0, intensity);
        (pt.clone(), color)
    });

    draw.polyline().weight(2.0).points_colored(points);
    // draw.line()
    //     .weight(2.0)
    //     .start(agent.pos)
    //     .end(*agent.history.front().unwrap())
    //     .color(WHITE);
}
