// ドキュメント
// https://docs.rs/nannou/latest/nannou/
//
// 色名一覧
// https://docs.rs/nannou/latest/nannou/color/named/index.html

use nannou::prelude::*;

// 1つのオブジェクトの例
// モジュール化する例としてわかりやすい
mod t001_ball;
use crate::t001_ball::Ball;

// 複数のオブジェクトを管理する1つのオブジェクトの例
// モジュール化する例としてわかりやすい
mod t001_particle_system;
use crate::t001_particle_system::ParticleSystem;

// 繰り返し処理で便利なのでとりあえず入れとこう
#[allow(unused_imports)]
use itertools::Itertools;

struct Model {
    any_value1: isize,  // 何か
    mouse_down: bool,   // 左クリックで true になる
    ball: Ball,         // 中央をまわる物体
    ps: ParticleSystem, // マウス位置から噴射する物体を管理してるやつ
}

impl Model {
    // Model 内の変数を操作するまとまった処理はここに書く
    fn any_value1_change(&mut self, sign: isize) {
        self.any_value1 += sign;
    }
}

fn main() {
    // お約束
    nannou::app(model).event(event).update(update).run();
}

fn model(app: &App) -> Model {
    // ここらへんもお約束
    app.new_window()
        .size(640, 480) // あとからは app.window_rect() でわかるので定数にしなくてもよかろう
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .view(view)
        .build()
        .unwrap();

    // Windowタイトルを入れる場合
    if true {
        app.main_window().set_title("Nannou all in one template");
    }

    Model {
        any_value1: 0,
        mouse_down: false,
        ball: Ball::new(DODGERBLUE),
        ps: ParticleSystem::new(),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(KeyPressed(key)),
        ..
    } = event
    {
        // 押しっぱなしにはならない。キーを叩いた瞬間にある処理を1回だけ行うとき用
        match key {
            Key::Z => {
                // todo
            }
            Key::X => {
                // todo
            }
            Key::C => {
                // todo
            }
            Key::Space => {
                // todo
            }
            Key::Left => model.any_value1_change(-1),
            Key::Right => model.any_value1_change(1),

            // よくわらないモード
            Key::Key0 => app.set_loop_mode(LoopMode::Wait),

            // Vsync同期がデフォルト
            Key::Key1 => app.set_loop_mode(LoopMode::RefreshSync),

            // 効いてなくない？
            Key::Key3 => app.set_loop_mode(LoopMode::rate_fps(30.0)),
            Key::Key6 => app.set_loop_mode(LoopMode::rate_fps(60.0)),

            // 1回だけ
            Key::P => app.set_loop_mode(LoopMode::loop_once()),

            // フルスクリーンは最初から Command + f で行える
            // Key::F     => app.main_window().set_fullscreen(true),

            Key::Q => app.quit(),

            // 画面キャプチャ
            Key::S => app
                .main_window()
                .capture_frame(app.exe_name().unwrap() + ".png"),

            _ => {}
        }
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.mouse_down = true,
        MouseButton::Right => {},
        _ => {}
    }
}

fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.mouse_down = false,
        MouseButton::Right => {},
        _ => {}
    }
}

// 何か動かすときの処理
fn update(app: &App, model: &mut Model, _update: Update) {
    // ボールを動かす
    if true {
        model.ball.update(&app);
    }

    // パーティクルを動かす
    if true {
        model.ps.origin = pt2(app.mouse.x, app.mouse.y);
        if (((app.time * 60.0) as usize) % 3) == 0 { // 3フレーム毎に1発
            model.ps.add_particle();
        }
        model.ps.update();
    }
}

// 表示する処理
// ここで更新はしない方がいい
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    // 全消去 (残像なし)
    if true {
        frame.clear(hsl(240.0 / 360.0, 1.0, 0.03));
    }

    // 全消去 (残像あり)
    if false {
        draw.rect()
            .wh(app.window_rect().wh())
            .hsla(240.0 / 360.0, 1.0, 0.03, 0.1);
    }

    // 左上からマウスに線
    if true {
        let start = win.top_left();
        let mouse = app.mouse.position();
        draw.line().points(start, mouse).color(DODGERBLUE);
    }

    // 左上に長方形
    if true {
        let r = Rect::from_w_h(100.0, 100.0);
        let r = r.top_left_of(win.pad(30.0)); // 余白
        draw.rect()
            .xy(r.xy())
            .wh(r.wh())
            .hsla(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(1.0) // 枠を書くときは必須
            .stroke(DODGERBLUE);
    }

    // 右下に円
    if true {
        let r = Rect::from_w_h(100.0, 100.0);
        let r = r.bottom_right_of(win.pad(30.0)); // 余白
        draw.ellipse()
            .xy(r.xy())
            .wh(r.wh())
            .hsla(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(1.0) // 枠を書くときは必須
            .stroke(DODGERBLUE);
    }

    // 左下に三角
    if true {
        let r = Rect::from_w_h(100.0, 100.0);
        let r = r.bottom_left_of(win.pad(30.0)); // 余白
        draw.tri()
            .points(r.bottom_left(), r.mid_top(), r.bottom_right())
            .hsla(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(1.0)
            // .rotate(app.time)
            .stroke(DODGERBLUE);
    }

    // 中央からマウスに矢印
    if true {
        draw.arrow()
            .start_cap_round()
            .weight(3.0)        // 太さ
            .points(vec2(0.0, 0.0), app.mouse.position())
            .color(DODGERBLUE);
    }

    // 中央から十字矢印
    if true {
        let crosshair_color = DODGERBLUE;
        let ends = [
            win.mid_top(),
            win.mid_right(),
            win.mid_bottom(),
            win.mid_left(),
        ];
        ends.iter().for_each(|&end| {
            draw.arrow()
                .start_cap_round()
                .head_length(16.0)
                .head_width(8.0)
                .color(crosshair_color)
                .end(end);
        })
    }

    // マウス座標をマウス位置に表示
    if true {
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(DODGERBLUE);
    }

    // マウスの衛星
    if true {
        let x = (app.time * 5.0 * 0.3).cos() * win.w() / 2.0;
        let y = (app.time * 6.0 * 0.3).sin() * win.h() / 2.0;
        // map_range を使う場合
        // let x = (app.time * 5.0 * 0.3).cos();
        // let y = (app.time * 6.0 * 0.3).sin();
        // let x = map_range(x, -1.0, 1.0, win.left(), win.right());
        // let y = map_range(y, -1.0, 1.0, win.bottom(), win.top());
        draw.ellipse().x_y(x, y).color(DODGERBLUE);
    }

    // 中央の5角形
    if true {
        let n = 5;
        let list = (0..n + 1).map(|i| {
            let angle = 2.0 * PI / (n as f32) * (i as f32);
            pt2(angle.sin(), angle.cos()) * 100.0
        });
        draw.polyline()
            .rotate(-app.time)
            .stroke_weight(5.0)
            .points(list)
            .color(DODGERBLUE);
    }

    // fn event 内ではなく view で押しているキーに応じて何かする場合
    if true {
        if app.keys.down.contains(&Key::Z) {
            println!("{:?}", app.keys.down);
        }
        if app.keys.down.contains(&Key::X) {
            println!("{:?}", app.keys.down);
        }
        if app.keys.down.contains(&Key::C) {
            println!("{:?}", app.keys.down);
        }
    }

    // 描画処理はここにかかずに委譲する例
    if true {
        model.ball.display(&draw);
        model.ps.draw(&draw);
    }

    // アプリ情報を表示
    if true {
        let r = Rect::from_w_h(win.w(), 22.0).bottom_left_of(win.pad(0.0));
        // draw.rect().xy(r.xy()).wh(r.wh()).rgb(0.0, 0.0, 0.4);
        let text = format!("{:?} {}", model.mouse_down, model.any_value1);
        draw.text(&text).xy(r.xy()).wh(r.wh()).color(CORNFLOWERBLUE);
    }

    // システム情報を表示
    if true {
        let r = Rect::from_w_h(win.w(), 22.0).top_left_of(win.pad(0.0));
        let text = format!(
            "{} {:.2} {:.0}fps {:?} {}x{} M({:.0},{:.0}) {:.0} ",
            frame.nth(),
            app.time,
            app.fps(),
            app.loop_mode(),
            win.w(),
            win.h(),
            app.mouse.x,
            app.mouse.y,
            app.mouse.position().length(),
        );
        draw.text(&text)
            .xy(r.xy())
            .wh(r.wh())
            .left_justify()
             // .align_text_top()
            .color(CORNFLOWERBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
