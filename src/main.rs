use std::collections::VecDeque;

use nannou::{prelude::*, rand::random_f32, ui::prelude::*};

fn main() {
    better_panic::install();
    nannou::app(model).update(update).run();
}

struct Agent {
    pos: Point2,
    dir: Vector2,
}

struct Model {
    agents: VecDeque<Agent>,
    last_spawn: f32,
    window: window::Id,
    ui: Ui,
    ids: Ids,
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
    Model {
        agents: VecDeque::with_capacity(100000),
        last_spawn: 0.0,
        window,
        ui,
        ids,
    }
}

fn random_agent() -> Agent {
    let dir = (vec2(random_f32(), random_f32()) - vec2(0.5, 0.5)) * 2.0;
    Agent {
        pos: pt2(0.0, 0.0),
        dir,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let should_spawn = app.time.floor() > model.last_spawn;

    if should_spawn {
        model.agents.push_back(random_agent());
    }

    let bounds = app.window(model.window).unwrap().rect();
    for agent in &mut model.agents {
        agent.pos += agent.dir;

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
        .down(20.0)
        .set(model.ids.agent_count, ui);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    for agent in &model.agents {
        draw.line()
            .start(agent.pos)
            .end(agent.pos - (agent.dir * 10.0))
            .weight(2.0)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();

    model.ui.draw_to_frame(app, &frame).unwrap();
}
