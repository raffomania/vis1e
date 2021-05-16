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
    config: Config,
}

#[derive(Clone, Copy)]
struct Config {
    speed: f32,
}

struct Model {
    agents: VecDeque<Agent>,
    window: window::Id,
    ui: Ui,
    ids: Ids,
    max_agents: u32,
    config: Config,
}

widget_ids! {
    struct Ids {
        fps,
        agent_count,
        agent_speed,
        max_agents
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
        max_agents: 50,
        config: Config { speed: 200.0 },
    }
}

fn random_agent(config: &Config) -> Agent {
    let r1 = random_f32().max(0.2);
    let r2 = random_f32().max(0.2);
    let dir = vec2(r1, r2) - vec2(0.5, 0.5);
    Agent {
        pos: pt2(0.0, 0.0),
        dir,
        config: config.clone(),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let bounds = app.window(model.window).unwrap().rect();

    model.agents.pop_front();

    if (model.agents.len() as u32) < model.max_agents {
        model.agents.push_back(random_agent(&model.config));
    }

    let dt = update.since_last.clone();
    model
        .agents
        .par_iter_mut()
        .for_each(|agent| update_agent(agent, &dt, &bounds));

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

    let max_agent_inputs = widget::Slider::new(model.max_agents as f32, 0.0, 10000.0)
        .w_h(200.0, 25.0)
        .down(10.0)
        .label("Max agents")
        .set(model.ids.max_agents, ui);
    for value in max_agent_inputs {
        model.max_agents = value as u32;
    }

    let speed_inputs = widget::Slider::new(model.config.speed, 0.0, 10000.0)
        .w_h(200.0, 25.0)
        .down(10.0)
        .label("Agent speed")
        .set(model.ids.agent_speed, ui);
    for value in speed_inputs {
        model.config.speed = value;
    }
}

fn update_agent(agent: &mut Agent, dt: &Duration, bounds: &nannou::prelude::Rect) {
    agent.pos += agent.dir * dt.as_secs_f32() * agent.config.speed;

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
    draw.line()
        .weight(5.0)
        .start(agent.pos - agent.dir * agent.config.speed)
        .end(agent.pos)
        .color(WHITE);
}
