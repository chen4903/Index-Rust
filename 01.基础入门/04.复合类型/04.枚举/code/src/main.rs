#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds('A');

   print_card(c1);
   print_card(c2);

   let _m1 = Message::Move { x: 12, y: 323 };
   let _m2 = Message::Write("Hello, I am LEVI_104".to_string());

}

fn print_card(card: PokerCard) {
    // 需要在定义 enum PokerCard 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",card);
}