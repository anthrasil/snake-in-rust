use macroquad::{ prelude::*, ui::{root_ui, widgets::Button, Skin}};
use ::rand::{rngs::ThreadRng, Rng};
fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}
struct Snake{
    body_parts:Vec<(f32,f32)>,
    part_size:u8,
}
impl Snake {
    fn new(body_parts:&[(f32,f32)],part_size:u8)->Self{
        Snake {body_parts: body_parts.to_vec(), part_size: part_size }
    }
    fn move_snake(&mut self,apple_eaten:bool,direction:u8) {
        if direction==0{
            return;
        }
        let length=self.body_parts.len();
        if apple_eaten{
            self.body_parts.push(self.body_parts[length-1]);
        }
        for i in (1..length).rev(){
            self.body_parts[i]=self.body_parts[i-1];
        }
        self.body_parts[0]=match direction {
            1 =>(self.body_parts[0].0+(1*self.part_size)as f32,self.body_parts[0].1),
            2 =>(self.body_parts[0].0,self.body_parts[0].1+(1*self.part_size)as f32),
            3 =>(self.body_parts[0].0-(1*self.part_size)as f32,self.body_parts[0].1),
            4 =>(self.body_parts[0].0,self.body_parts[0].1-(1*self.part_size)as f32),
            _ => self.body_parts[0],
        }
    }
    fn draw_snake(&self) {
        let half_part_size=(self.part_size as f32)/2f32;
        draw_circle(self.body_parts[0].0+half_part_size, self.body_parts[0].1+half_part_size,  half_part_size*1.35, GREEN);
        for i in 1..self.body_parts.len() {
            draw_rectangle(self.body_parts[i].0, self.body_parts[i].1, self.part_size as f32, self.part_size as f32, GREEN);
        }
    }
    fn dead(&self,window_size:f32)->bool {
        let head=self.body_parts[0];
        if head.0<0f32||head.1<0f32||head.0>=window_size||head.1>=window_size{
            return true;
        }
        for i in 1..self.body_parts.len(){
            if head==self.body_parts[i] {
                return true;
            }
        }
        false
    }
}
struct Apple{
    position:(f32,f32),
    size:u8,
}
impl Apple {
    fn new(start_position:(f32,f32),size:u8)->Self{
        Apple { position: start_position,size:size/2 }
    }
    fn draw_apple(&self) {
        draw_circle(self.position.0+(self.size) as f32, self.position.1+(self.size) as f32, self.size as f32, RED);
    }
    fn eaten(&mut self,head:&(f32,f32))->bool {
        if self.position==*head{
            return true;
        }
        false
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let window_size=screen_width();
    let mut rng=::rand::rng();
    let tile_size=200;
    let movement_cooldown=tile_size as f64/200f64;
    let body_parts=vec![
        (tile_size as f32,0.0),(tile_size as f32*2f32,0.0),(tile_size as f32*3f32,0.0)
    ];
    let main_button_size=Vec2::from([window_size/4f32,window_size/8f32]);
    let main_button_position=Vec2::from([window_size/2f32-main_button_size.x/2f32,window_size/2.5f32-main_button_size.y/2f32]);
    let button_skin = {
        let mut skin = root_ui().default_skin();

        skin.button_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(255, 0, 0, 255))
            .text_color_hovered(Color::from_rgba(255, 0, 0, 200))
            .text_color_clicked(Color::from_rgba(255, 0, 0, 100))
            .color(Color::from_rgba(25, 25, 25, 255))
            .color_hovered(Color::from_rgba(25, 25, 25, 200))
            .color_clicked(Color::from_rgba(25, 25, 25, 100))
            .font_size((main_button_size.y/1.5f32) as u16)
            .build();

        skin
    };
    let mut won=false;
    loop {
        if run_start_screen(&button_skin, &main_button_position, &main_button_size).await {
            break;
        }
        won=run_game(window_size, &mut rng, tile_size, movement_cooldown as f64, &body_parts).await;
        run_end_screen(&button_skin, &main_button_position, &main_button_size, won).await;
        won=false;
        next_frame().await;   
    }
}
fn generate_random_apple_position(rng: &mut ThreadRng,window_size:u32,tile_size:u32)->(f32,f32) {
    ((rng.random_range(0..(window_size/tile_size))*tile_size) as f32,(rng.random_range(0..(window_size/tile_size))*tile_size) as f32)
}
async fn run_game(window_size:f32,rng:&mut ThreadRng,tile_size:u8,movement_cooldown:f64,body_parts:&Vec<(f32,f32)>)->bool {
    let mut direction=0;
    let mut snake=Snake::new(body_parts, tile_size);
    let mut apple=Apple::new(generate_random_apple_position( rng,window_size as u32,tile_size as u32),tile_size);
    let mut last_action_time=get_time();
    loop {
        if is_key_down(KeyCode::Right) {
            direction=1;
        }else if is_key_down(KeyCode::Down) {
            direction=2;
        } else if is_key_down(KeyCode::Left) {
            direction=3;
        }else if is_key_down(KeyCode::Up) {
            direction=4;
        }
        let now=get_time();
        if now-last_action_time>=movement_cooldown {
            clear_background(BLACK);
            let apple_eaten=apple.eaten(&snake.body_parts[0]);
            if apple_eaten{
                apple.position=generate_random_apple_position(rng, window_size as u32, tile_size as u32);
                //println!("{:?}",apple.position);
            }
            snake.move_snake(apple_eaten, direction);
            if snake.dead(window_size){
                return false;
            }
            if snake.body_parts.len()>=((window_size/(tile_size as f32))*(window_size/(tile_size as f32))) as usize {
                return  true;
            }
            last_action_time=now
        }
        apple.draw_apple();
        snake.draw_snake();
        next_frame().await;
    }
}
async fn run_start_screen(button_skin:&Skin,button_position:&Vec2,button_size:&Vec2)->bool {
    root_ui().push_skin(button_skin);
    loop {
        let start_button=Button::new("Start")
        .position(*button_position)
        .size(*button_size)
        .ui(&mut root_ui());
        let close_button=Button::new("Close")
        .position(vec2(button_position.x, button_position.y+button_size.y*1.1))
        .size(*button_size)
        .ui(&mut root_ui());
        if start_button {
            root_ui().pop_skin();
            return false;
        }
        if close_button {
            root_ui().pop_skin();
            return true;
        }
        next_frame().await;   
    }   
}
async fn run_end_screen(button_skin:&Skin,button_position:&Vec2,button_size:&Vec2,won:bool) {
    let mut text=String::from("You Lost");
    let mut text_color=RED;
    if won {
        text=String::from("You won");
        text_color=GREEN;
    }
    root_ui().push_skin(button_skin);
    loop {
        draw_text(&text, button_size.x*2f32-button_size.y*3f32, button_size.y*2f32, button_size.y*2f32, text_color);
        let menu_button=Button::new("Menu")
        .position(*button_position)
        .size(*button_size)
        .ui(&mut root_ui());
        if menu_button {
            break;
        }
        next_frame().await;
    }
}